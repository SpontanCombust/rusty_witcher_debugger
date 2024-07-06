use std::{net::Ipv4Addr, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, time::Duration};

use rw3d_mock_server::MockWitcherServer;
use rw3d_net::{connection::WitcherConnection, messages::{notifications::*, requests::*}};
use rw3d_net_client::WitcherClient;



#[test]
fn integration_test() -> anyhow::Result<()> {
    let server_cancel_token = Arc::new(AtomicBool::new(false));
    let cancel_token_cloned = server_cancel_token.clone();
    let server_handle = std::thread::spawn(move || -> anyhow::Result<()> {
        let server = MockWitcherServer::new()?;
        server.listen(cancel_token_cloned);
        Ok(())
    });

    let conn = WitcherConnection::connect_timeout(Ipv4Addr::LOCALHOST.into(), Duration::from_secs(1))?;
    let client = WitcherClient::new(conn);
    client.start()?;

    let packets_received = Arc::new(AtomicUsize::new(0));
    let packets_received_cl = packets_received.clone();
    client.on_raw_packet(move |_| {
        packets_received_cl.fetch_add(1, Ordering::Relaxed);
    });

    let (finish_reload, did_finish_reload) = std::sync::mpsc::channel();
    client.on_scripts_reload_progress(move |params| {
        if let ScriptsReloadProgressParams::Finished { .. } = params {
            finish_reload.send(()).unwrap();
        }
    });

    client.reload_scripts()?;
    assert!(did_finish_reload.recv_timeout(Duration::from_secs(5)).is_ok());

    
    client.scripts_root_path()?;


    client.execute_command(ExecuteCommandParams { 
        cmd: "spawnt(12)".into() 
    })?;


    client.script_packages()?;


    client.opcodes(OpcodesParams {
        func_name: "additem".into(),
        class_name: None
    })?;


    client.config_vars(ConfigVarsParams {
        section_filter: Some("boat".into()),
        name_filter: Some("yaw".into())
    })?;

    assert!(packets_received.load(Ordering::Relaxed) >= 7);


    client.stop()?;

    server_cancel_token.store(true, Ordering::Relaxed);
    server_handle.join().unwrap()?;

    Ok(())
}