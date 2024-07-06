use std::{collections::VecDeque, marker::PhantomData, sync::{atomic::AtomicBool, Arc, Mutex}};

use anyhow::Context;
use dashmap::DashMap;
use rw3d_net::{connection::WitcherConnection, messages::*, protocol::WitcherPacket};


pub(crate) struct Router {
    id_registry: Mutex<MessageIdRegistry>,
    raw_packet_handler: Mutex<Option<Box<dyn RouteHandler + Send + Sync>>>,
    response_handlers: DashMap<MessageId, VecDeque<Box<dyn RouteHandler + Send + Sync>>>,
    notif_handlers: DashMap<MessageId, Box<dyn RouteHandler + Send + Sync>>,
}

impl Router {
    const DEFAULT_POLL_RATE_MILLIS: u64 = 500;

    pub fn new() -> Self {
        Self {
            id_registry: Mutex::new(MessageIdRegistry::new()),
            raw_packet_handler: Mutex::new(None),
            response_handlers: DashMap::new(),
            notif_handlers: DashMap::new()
        }
    }   

    pub fn add_response_callback<R, F>(&self, callback: F) 
    where R: Response + Send + Sync + 'static,
          F: FnOnce(R::Body) + Send + Sync + 'static {
        
        let id = self.id_registry.lock().unwrap().register_message::<R>();
        self.response_handlers.entry(id)
            .or_default()
            .push_back(Box::new(ResponseRouteHandler::<R, F>::new(callback)));
    }

    pub fn set_notification_callback<N, F>(&self, callback: F) 
    where N: Notification + Send + Sync + 'static,
          F: FnMut(N::Body) + Send + Sync + 'static {
        
        let id = self.id_registry.lock().unwrap().register_message::<N>();
        self.notif_handlers.entry(id)
            .insert(Box::new(NotificationRouteHandler::<N, F>::new(callback)));
    }

    pub fn set_raw_packet_callback<F>(&self, callback: F)
    where F: FnMut(WitcherPacket) + Send + Sync + 'static {
        let mut raw_handler = self.raw_packet_handler.lock().unwrap();
        *raw_handler = Some(Box::new(RawRouteHandler::new(callback)));
    }

    pub fn event_loop(&self, mut read_conn: WitcherConnection, poll_rate: Option<std::time::Duration>, cancel_token: Arc<AtomicBool>) -> anyhow::Result<()> {
        read_conn.set_nonblocking(true)?;
        let poll_rate = poll_rate.unwrap_or(std::time::Duration::from_millis(Self::DEFAULT_POLL_RATE_MILLIS));

        loop {
            if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            if read_conn.peek()? {
                let packet = read_conn.receive()?;
                {
                    let mut raw_handler = self.raw_packet_handler.lock().unwrap();
                    if let Some(raw_handler) = &mut *raw_handler {
                        raw_handler.accept_packet(packet.clone())?;
                    }
                }
                if let Some(id) = self.id_registry.lock().unwrap().probe_message_id(&packet) {
                    if let Some(mut nh) = self.notif_handlers.get_mut(&id) {
                        nh.accept_packet(packet)?;
                    }
                    else if let Some(mut rhs) = self.response_handlers.get_mut(&id) {
                        if let Some(mut rh) = rhs.pop_front() {
                            rh.accept_packet(packet)?;
                        }
                    }
                }
            } else {
                std::thread::sleep(poll_rate);
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("id_registry", &self.id_registry)
            .field("raw_handler", &self.raw_packet_handler.lock().unwrap().is_some())
            .field("response_handlers", &self.response_handlers.iter().map(|h| h.key().to_owned()).collect::<Vec<_>>())
            .field("notif_handlers", &self.notif_handlers.iter().map(|h| h.key().to_owned()).collect::<Vec<_>>())
            .finish()
    }
}



trait RouteHandler {
    fn accept_packet(&mut self, packet: WitcherPacket) -> anyhow::Result<()>;
}


struct NotificationRouteHandler<N, F> {
    notif_callback: F,
    notif_phantom: PhantomData<N>
}

impl<N, F> NotificationRouteHandler<N, F>
where N: Notification, F: FnMut(N::Body) {
    fn new(notif_callback: F) -> Self {
        Self {
            notif_callback,
            notif_phantom: PhantomData
        }
    }
}

impl<N, F> RouteHandler for NotificationRouteHandler<N, F> 
where N: Notification, F: FnMut(N::Body) {
    fn accept_packet(&mut self, packet: WitcherPacket) -> anyhow::Result<()> {
        let notif = N::disassemble_packet(packet).context("Notification deserialization error")?;
        (self.notif_callback)(notif);
        Ok(())
    }
}


struct ResponseRouteHandler<R, F> {
    resp_callback: Option<F>,
    resp_phantom: PhantomData<R>
}

impl<R, F> ResponseRouteHandler<R, F>
where R: Response, F: FnOnce(R::Body) {
    fn new(resp_callback: F) -> Self {
        Self {
            resp_callback: Some(resp_callback),
            resp_phantom: PhantomData
        }
    }
}

impl<R, F> RouteHandler for ResponseRouteHandler<R, F> 
where R: Response, F: FnOnce(R::Body) {
    fn accept_packet(&mut self, packet: WitcherPacket) -> anyhow::Result<()> {
        let resp = R::disassemble_packet(packet).context("Response deserialization error")?;
        if let Some(resp_handler) = self.resp_callback.take() {
            (resp_handler)(resp);
        }
        Ok(())
    }
}


struct RawRouteHandler<F> {
    raw_packet_callback: F,
}

impl<F> RawRouteHandler<F>
where F: FnMut(WitcherPacket) {
    fn new(raw_packet_callback: F) -> Self {
        Self {
            raw_packet_callback
        }
    }
}

impl<F> RouteHandler for RawRouteHandler<F>
where F: FnMut(WitcherPacket) {
    fn accept_packet(&mut self, packet: WitcherPacket) -> anyhow::Result<()> {
        (self.raw_packet_callback)(packet);
        Ok(())
    }
}