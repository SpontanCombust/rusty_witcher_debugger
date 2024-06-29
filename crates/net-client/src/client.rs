use std::sync::{atomic::AtomicBool, Arc, Mutex};

use anyhow::{bail, Context};
use rw3d_net::{connection::WitcherConnection, messages::{notifications::Notification, requests::Request, Message}};

use crate::Router;


#[derive(Debug)]
pub struct WitcherClient {
    write_conn: Mutex<WitcherConnection>,
    router: Arc<Router>,
    router_thread: Mutex<Option<std::thread::JoinHandle<anyhow::Result<()>>>>,
    router_cancel_token: Arc<AtomicBool>
}

impl WitcherClient {
    #[inline]
    pub fn new(conn: WitcherConnection) -> Self {
        Self {
            write_conn: Mutex::new(conn),
            router: Arc::new(Router::new()),
            router_thread: Mutex::new(None),
            router_cancel_token: Arc::new(AtomicBool::new(false))
        }
    }


    pub fn send_notification<N>(&self, params: N::Body) -> anyhow::Result<()> 
    where N: Notification + Send + Sync + 'static {
        let packet = N::assemble_packet(params);
        self.write_conn.lock().unwrap().send(packet)?;
        Ok(())
    }

    pub fn send_request<R, F>(&self, params: R::Body, callback: F) -> anyhow::Result<()>
    where R: Request + Send + Sync + 'static,
          F: FnOnce(<R::Response as Message>::Body) + Send + Sync + 'static,
          R::Response: Send + Sync {
        
        let packet = R::assemble_packet(params);
        self.write_conn.lock().unwrap().send(packet)?;

        self.router.add_response_handler::<R::Response, F>(callback);
        Ok(())
    }

    pub fn on_notification<N, F>(&self, callback: F) 
    where N: Notification + Send + Sync + 'static,
          F: Fn(N::Body) + Send + Sync + 'static {

        self.router.set_notification_handler::<N, F>(callback);
    }


    /// Will error if the client was already started before
    pub fn start(&self) -> anyhow::Result<()> {
        let mut current_router_thread = self.router_thread.lock().unwrap();
        if current_router_thread.is_some() {
            bail!("Client has been started before")
        }

        let router = self.router.clone();
        let cancel_token = self.router_cancel_token.clone();
        let read_conn = self.write_conn.lock().unwrap().try_clone().context("Failed to clone the connection")?;

        let router_thread = std::thread::spawn(move || router.event_loop(read_conn, cancel_token));
        *current_router_thread = Some(router_thread);
        Ok(())
    }

    pub fn was_started(&self) -> bool {
        self.router_thread.lock().unwrap().is_some()
    }

    /// Will panic if the client has not been started yet
    pub fn stop(&self) -> std::thread::JoinHandle<anyhow::Result<()>> {
        let mut current_router_thread = self.router_thread.lock().unwrap();
        if current_router_thread.is_none() {
            panic!("Client has not been started yet");
        }

        current_router_thread.take().unwrap()
    }
}