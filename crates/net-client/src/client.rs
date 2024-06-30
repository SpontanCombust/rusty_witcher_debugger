use std::sync::{atomic::AtomicBool, Arc, Mutex};

use anyhow::{bail, Context};
use rw3d_net::{connection::WitcherConnection, messages::{notifications::*, requests::*, Message, WitcherNamespace}};

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

    /// Will error if the client has not been started yet
    pub fn stop(&self) -> anyhow::Result<()> {
        let mut current_router_thread = self.router_thread.lock().unwrap();
        if current_router_thread.is_none() {
            bail!("Client thread has not been started yet");
        }

        self.router_cancel_token.store(true, std::sync::atomic::Ordering::Relaxed);
        match current_router_thread.take().unwrap().join() {
            Ok(result) => result,
            Err(join_err) => {
                bail!("Client thread panicked: {:?}", join_err)
            }
        }
    }



    #[inline]
    pub fn listen_to_namespace(&self, params: ListenToNamespaceParams) -> anyhow::Result<()> {
        self.send_notification::<ListenToNamespace>(params)
    }

    pub fn listen_to_all_namespaces(&self) -> anyhow::Result<()> {
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::Config
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::Remote
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptCompiler
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptDebugger
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptProfiler
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::Scripts
        })?;
        self.send_notification::<ListenToNamespace>(ListenToNamespaceParams {
            namesp: WitcherNamespace::Utility
        })?;

        Ok(())
    }

    #[inline]
    pub fn reload_scripts(&self) -> anyhow::Result<()> {
        self.send_notification::<ReloadScripts>(())
    }

    #[inline]
    pub fn on_scripts_reload_progress<F>(&self, callback: F)
    where F: Fn(ScriptsReloadProgressParams) + Send + Sync + 'static {
        self.on_notification::<ScriptsReloadProgress, F>(callback)
    }

    #[inline]
    pub fn scripts_root_path(&self) -> anyhow::Result<ScriptsRootPathResult> {
        self.send_request::<ScriptsRootPath>(())
    }

    #[inline]
    pub fn execute_command(&self, params: ExecuteCommandParams) -> anyhow::Result<ExecuteCommandResult> {
        self.send_request::<ExecuteCommand>(params)
    }

    #[inline]
    pub fn script_packages<F>(&self) -> anyhow::Result<ScriptPackagesResult> {
        self.send_request::<ScriptPackages>(())
    }

    #[inline]
    pub fn opcodes<F>(&self, params: OpcodesParams) -> anyhow::Result<OpcodesResult> {
        self.send_request::<Opcodes>(params)
    }

    #[inline]
    pub fn config_vars<F>(&self, params: ConfigVarsParams) -> anyhow::Result<ConfigVarsResult> {
        self.send_request::<ConfigVars>(params)
    }


    
    fn send_notification<N>(&self, params: N::Body) -> anyhow::Result<()> 
    where N: Notification + Send + Sync + 'static {
        let packet = N::assemble_packet(params);
        self.write_conn.lock().unwrap().send(packet)?;
        Ok(())
    }

    fn send_request<R>(&self, params: R::Body) -> anyhow::Result<<R::Response as Message>::Body>
    where R: Request + Send + Sync + 'static,
          R::Response: Send + Sync + 'static,
          <R::Response as Message>::Body: Send {

        let read_timeout = self.write_conn.lock().unwrap().get_read_timeout()?;          
        let (send, recv) = std::sync::mpsc::channel();
        let result_sender = move |result: <R::Response as Message>::Body| {
            send.send(result).unwrap()
        };
                
        let packet = R::assemble_packet(params);
        self.router.add_response_handler::<R::Response, _>(result_sender);
        self.write_conn.lock().unwrap().send(packet)?;
        
        let result = if let Some(timeout) = read_timeout {
            recv.recv_timeout(timeout)?
        } else {
            recv.recv()?
        };

        Ok(result)
    }

    fn on_notification<N, F>(&self, callback: F) 
    where N: Notification + Send + Sync + 'static,
          F: Fn(N::Body) + Send + Sync + 'static {

        self.router.set_notification_handler::<N, F>(callback);
    }
}