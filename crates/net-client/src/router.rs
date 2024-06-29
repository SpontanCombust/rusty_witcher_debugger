use std::{collections::VecDeque, marker::PhantomData, sync::{atomic::AtomicBool, Arc, Mutex}};

use anyhow::Context;
use dashmap::DashMap;
use rw3d_net::{connection::WitcherConnection, messages::*, protocol::WitcherPacket};


pub struct Router {
    id_registry: MessageIdRegistry,
    response_handlers: DashMap<MessageId, VecDeque<Box<dyn RouteHandler + Send + Sync>>>,
    notif_handlers: DashMap<MessageId, Box<dyn RouteHandler + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        let mut id_registry = MessageIdRegistry::new();
        id_registry.register_server_messages();

        Self {
            id_registry,
            response_handlers: DashMap::new(),
            notif_handlers: DashMap::new()
        }
    }   

    pub fn add_response_handler<R, F>(&self, handler: F) 
    where R: Response + Send + Sync + 'static,
          F: FnOnce(R::Body) + Send + Sync + 'static {
        
        let id = R::assemble_id();
        self.response_handlers.entry(id)
            .or_default()
            .push_back(Box::new(ResponseRouteHandler::<R, F>::new(handler)));
    }

    pub fn set_notification_handler<N, F>(&self, handler: F) 
    where N: Notification + Send + Sync + 'static,
          F: Fn(N::Body) + Send + Sync + 'static {
        
        let id = N::assemble_id();
        self.notif_handlers.entry(id)
            .insert(Box::new(NotificationRouteHandler::<N, F>::new(handler)));
    }

    pub fn event_loop(&self, mut read_conn: WitcherConnection, cancel_token: Arc<AtomicBool>) -> anyhow::Result<()> {
        loop {
            if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            if read_conn.peek()? {
                let packet = read_conn.receive()?;
                if let Some(id) = self.id_registry.probe_message_id(&packet) {
                    if let Some(nh) = self.notif_handlers.get(&id) {
                        nh.accept_packet(packet)?;
                    }
                    else if let Some(mut rhs) = self.response_handlers.get_mut(&id) {
                        if let Some(rh) = rhs.pop_front() {
                            rh.accept_packet(packet)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("id_registry", &self.id_registry)
            .field("response_handlers", &self.response_handlers.iter().map(|h| h.key().to_owned()).collect::<Vec<_>>())
            .field("notif_handlers", &self.notif_handlers.iter().map(|h| h.key().to_owned()).collect::<Vec<_>>())
            .finish()
    }
}



trait RouteHandler {
    fn accept_packet(&self, packet: WitcherPacket) -> anyhow::Result<()>;
}


struct NotificationRouteHandler<N, F> {
    notif_handler: F,
    notif_phantom: PhantomData<N>
}

impl<N, F> NotificationRouteHandler<N, F>
where N: Notification, F: Fn(N::Body) {
    fn new(notif_handler: F) -> Self {
        Self {
            notif_handler,
            notif_phantom: PhantomData
        }
    }
}

impl<N, F> RouteHandler for NotificationRouteHandler<N, F> 
where N: Notification, F: Fn(N::Body) {
    fn accept_packet(&self, packet: WitcherPacket) -> anyhow::Result<()> {
        let notif = N::disassemble_packet(packet).context("Notification deserialization error")?;
        (self.notif_handler)(notif);
        Ok(())
    }
}


struct ResponseRouteHandler<R, F> {
    resp_handler: Mutex<Option<F>>,
    resp_phantom: PhantomData<R>
}

impl<R, F> ResponseRouteHandler<R, F>
where R: Response, F: FnOnce(R::Body) {
    fn new(resp_handler: F) -> Self {
        Self {
            resp_handler: Mutex::new(Some(resp_handler)),
            resp_phantom: PhantomData
        }
    }
}

impl<R, F> RouteHandler for ResponseRouteHandler<R, F> 
where R: Response, F: FnOnce(R::Body) {
    fn accept_packet(&self, packet: WitcherPacket) -> anyhow::Result<()> {
        let resp = R::disassemble_packet(packet).context("Response deserialization error")?;
        if let Some(resp_handler) = self.resp_handler.lock().unwrap().take() {
            (resp_handler)(resp);
        }
        Ok(())
    }
}