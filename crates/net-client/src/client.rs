use std::sync::{atomic::AtomicBool, Arc, Mutex};

use anyhow::{bail, Context};
use rw3d_net::{connection::WitcherConnection, messages::{notifications::*, requests::*, Message, WitcherNamespace}, protocol::WitcherPacket};

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

    /// Initializes communication with the server.
    /// This should be coupled with the call to [`Self::stop`] at the end of client's lifetime.
    /// 
    /// Will error if the client was already started before.
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

        self.listen_to_all_namespaces()?;

        Ok(())
    }

    pub fn was_started(&self) -> bool {
        self.router_thread.lock().unwrap().is_some()
    }

    /// Stop communication with the server.
    /// 
    /// Will error if the client has not been started yet.
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


    /// Set a callback that will be ivoked on every raw packet received from the server.
    #[inline]
    pub fn on_raw_packet<F>(&self, callback: F)
    where F: FnMut(WitcherPacket) + Send + Sync + 'static {
        self.router.set_raw_packet_callback(callback)
    }

    /// Send a notification to the server to recompile scripts.
    #[inline]
    pub fn reload_scripts(&self) -> anyhow::Result<()> {
        self.send_notification::<ReloadScripts>(())
    }

    /// Set a callback that will be invoked for script recompilation progress notifications sent from the server.
    #[inline]
    pub fn on_scripts_reload_progress<F>(&self, callback: F)
    where F: FnMut(ScriptsReloadProgressParams) + Send + Sync + 'static {
        self.on_notification::<ScriptsReloadProgress, F>(callback)
    }

    /// Send a request for the path to content0's script root.
    /// 
    /// Will block until the response is received or client waits for too long (based on connection's read_timeout).
    #[inline]
    pub fn scripts_root_path(&self) -> anyhow::Result<ScriptsRootPathResult> {
        self.send_request::<ScriptsRootPath>(())
    }

    /// Send a request to execute an exec function.
    /// 
    /// Will block until the response is received or client waits for too long (based on connection's read_timeout).
    #[inline]
    pub fn execute_command(&self, params: ExecuteCommandParams) -> anyhow::Result<ExecuteCommandResult> {
        self.send_request::<ExecuteCommand>(params)
    }

    /// Send a request to retrieve information about script packages detected by the game (vanilla and modded).
    /// 
    /// Will block until the response is received or client waits for too long (based on connection's read_timeout).
    #[inline]
    pub fn script_packages(&self) -> anyhow::Result<ScriptPackagesResult> {
        self.send_request::<ScriptPackages>(())
    }

    /// Send a request for the breakdown of opcodes in a function.
    /// 
    /// Will block until the response is received or client waits for too long (based on connection's read_timeout).
    #[inline]
    pub fn opcodes(&self, params: OpcodesParams) -> anyhow::Result<OpcodesResult> {
        self.send_request::<Opcodes>(params)
    }

    /// Send a request for the list of internal configuration vars.
    /// 
    /// Will block until the response is received or client waits for too long (based on connection's read_timeout).
    #[inline]
    pub fn config_vars(&self, params: ConfigVarsParams) -> anyhow::Result<ConfigVarsResult> {
        self.send_request::<ConfigVars>(params)
    }



    /// Notify the server to send back messages concerning given namespaces.
    /// This is necessary for responses to be received by the client.
    /// 
    /// To listen to all namespaces use [`Self::listen_to_all_namespaces`].
    #[inline]
    fn listen_to_namespace(&self, params: ListenToNamespaceParams) -> anyhow::Result<()> {
        self.send_notification::<ListenToNamespace>(params)
    }

    /// Notify the server to send back messages.
    /// This is necessary for responses to be received by the client and should be called after creating it.
    fn listen_to_all_namespaces(&self) -> anyhow::Result<()> {
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::Config
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::Remote
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptCompiler
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptDebugger
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::ScriptProfiler
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::Scripts
        })?;
        self.listen_to_namespace(ListenToNamespaceParams {
            namesp: WitcherNamespace::Utility
        })?;

        Ok(())
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
        self.router.add_response_callback::<R::Response, _>(result_sender);
        self.write_conn.lock().unwrap().send(packet)?;
        
        let result = recv.recv_timeout(read_timeout).context("Waited too long for the response")?;

        Ok(result)
    }

    fn on_notification<N, F>(&self, callback: F) 
    where N: Notification + Send + Sync + 'static,
          F: FnMut(N::Body) + Send + Sync + 'static {

        self.router.set_notification_callback::<N, F>(callback);
    }
}


impl Drop for WitcherClient {
    fn drop(&mut self) {
        // if the client was already stopped this will return Err but nothing besides that
        let _ = self.stop();
    }
}