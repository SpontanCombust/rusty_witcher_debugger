use std::{collections::HashMap, net::{Ipv4Addr, TcpListener, TcpStream}, sync::{atomic::AtomicBool, Arc}};

use rw3d_net::{connection::WitcherConnection, messages::{notifications::*, requests::*, Message, MessageId, MessageIdRegistry}, protocol::{Decode, Encode, WitcherPacket}};


pub struct MockWitcherServer {
    listener: TcpListener,
    id_registry: MessageIdRegistry,
    services: ServiceMap
}

type ServiceMap = HashMap<MessageId, Box<dyn Service + Send + Sync>>;

impl MockWitcherServer {
    pub fn new() -> anyhow::Result<Arc<Self>> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, WitcherConnection::GAME_PORT))?;
        listener.set_nonblocking(true).unwrap();

        let mut id_registry = MessageIdRegistry::new();
        let mut services = ServiceMap::new();
        
        let id = id_registry.register_message::<ListenToNamespace>();
        services.insert(id, Box::new(ListenToNamespaceService));

        let id = id_registry.register_message::<ReloadScripts>();
        services.insert(id, Box::new(ReloadScriptsService));

        let id = id_registry.register_message::<ScriptsRootPath>();
        services.insert(id, Box::new(ScriptsRootPathService));

        let id = id_registry.register_message::<ExecuteCommand>();
        services.insert(id, Box::new(ExecuteCommandService));

        let id = id_registry.register_message::<ScriptPackages>();
        services.insert(id, Box::new(ScriptPackagesService));

        let id = id_registry.register_message::<Opcodes>();
        services.insert(id, Box::new(OpcodesService));

        let id = id_registry.register_message::<ConfigVars>();
        services.insert(id, Box::new(ConfigVarsService));

        Ok(Arc::new(Self {
            listener,
            id_registry,
            services
        }))
    }


    pub fn listen(self: Arc<Self>, cancel_token: Arc<AtomicBool>) {
        println!("Server listening on port {}", WitcherConnection::GAME_PORT);

        loop {
            if cancel_token.load(std::sync::atomic::Ordering::Relaxed) {
                println!("Shutting down the server...");
                break;
            }

            match self.listener.accept() {
                Ok((socket, addr)) => {
                    println!("Client connected on address {}", addr);
    
                    let self_clone = self.clone();
                    std::thread::spawn(move || -> anyhow::Result<()> {
                        self_clone.serve_for(socket)
                    });
                }
                Err(err) if err.kind() != std::io::ErrorKind::WouldBlock => {
                    eprintln!("Client failed to connect: {}", err);
                },
                _ => {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    }

    pub fn serve_for(&self, mut client_socket: TcpStream) -> anyhow::Result<()> {
        loop {
            let packet = WitcherPacket::decode_from(&mut client_socket)?;
            if let Some(service) = self.id_registry.probe_message_id(&packet).and_then(|id| self.services.get(&id)) {
                service.accept_packet(packet, &mut client_socket);
            }
        }
    }
}


trait Service {
    fn accept_packet(&self, packet: WitcherPacket, socket: &mut TcpStream);
}


struct ListenToNamespaceService;

impl Service for ListenToNamespaceService {
    fn accept_packet(&self, _packet: WitcherPacket, _socket: &mut TcpStream) {
        println!("Handling ListenToNamespace notification...");
        // notification, nothing is expected to be sent back
    }
} 


struct ReloadScriptsService;

impl Service for ReloadScriptsService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling ReloadScripts notification...");

        ScriptsReloadProgress::assemble_packet(ScriptsReloadProgressParams::Started).encode_into(socket).unwrap();
        ScriptsReloadProgress::assemble_packet(ScriptsReloadProgressParams::Log { 
            message: "Compiling foo.ws".into() 
        }).encode_into(socket).unwrap();
        ScriptsReloadProgress::assemble_packet(ScriptsReloadProgressParams::Log { 
            message: "Compiling bar.ws".into() 
        }).encode_into(socket).unwrap();
        ScriptsReloadProgress::assemble_packet(ScriptsReloadProgressParams::Warn { 
            line: 12,
            local_script_path: "bar.ws".into(),
            message: "Variable declared, but unused".into() 
        }).encode_into(socket).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(1));

        ScriptsReloadProgress::assemble_packet(ScriptsReloadProgressParams::Finished { 
            success: true
        }).encode_into(socket).unwrap();
    }
}


struct ScriptsRootPathService;

impl Service for ScriptsRootPathService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling ScriptsRootPath request...");

        ScriptsRootPathResponse::assemble_packet(ScriptsRootPathResult {
            abs_path: r"C:\GOG\Witcher 3\content\content0\scripts".into()
        }).encode_into(socket).unwrap();
    }
}


struct ExecuteCommandService;

impl Service for ExecuteCommandService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling ExecuteCommand request...");

        ExecuteCommandResponse::assemble_packet(ExecuteCommandResult::Success { 
            log_output: None
        }).encode_into(socket).unwrap();
    }
}


struct ScriptPackagesService;

impl Service for ScriptPackagesService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling ScriptPackages request...");

        ScriptPackagesResponse::assemble_packet(ScriptPackagesResult {
            packages: vec![
                ScriptPackageInfo {
                    abs_scripts_root_path: r"C:\GOG\Witcher 3\content\content0\scripts".into(),
                    package_name: "content0".into()
                },
                ScriptPackageInfo {
                    abs_scripts_root_path: r"C:\GOG\Witcher 3\Mods\mod000_MergedFiles\content\scripts".into(),
                    package_name: "mod000_MergedFiles".into()
                },
                ScriptPackageInfo {
                    abs_scripts_root_path: r"C:\GOG\Witcher 3\Mods\modSharedImports\content\scripts".into(),
                    package_name: "modSharedImports".into()
                },
                ScriptPackageInfo {
                    abs_scripts_root_path: r"C:\GOG\Witcher 3\Mods\modBrothersInArms\content\scripts".into(),
                    package_name: "modBrothersInArms".into()
                }
            ]
        }).encode_into(socket).unwrap();
    }
}


struct OpcodesService;

impl Service for OpcodesService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling Opcodes request...");

        OpcodesResponse::assemble_packet(OpcodesResult {
            breakdowns: vec![
                OpcodeBreakdown {
                    line: 0,
                    opcodes: vec![
                        "00000152F45B8361 (   1): Return".into(),
                        "00000152F45B8362 (   2): ObjectToBool".into(),
                    ],
                },
                OpcodeBreakdown {
                    line: 1364,
                    opcodes: vec![
                        "00000152F45B8363 (   3): Breakpoint".into(),
                        "00000152F45B8369 (   9): DynamicCast".into(),
                        "00000152F45B8372 (  18): This".into(),
                        "00000152F45B8373 (  19): Nop".into(),
                    ]
                }
            ]
        }).encode_into(socket).unwrap();
    }
}


struct ConfigVarsService;

impl Service for ConfigVarsService {
    fn accept_packet(&self, _packet: WitcherPacket, socket: &mut TcpStream) {
        println!("Handling ConfigVars request...");

        ConfigVarsResponse::assemble_packet(ConfigVarsResult {
            vars: vec![
                ConfigVarInfo {
                    section: "Visuals".into(),
                    name: "GammaValue".into(),
                    value: "1".into(),
                    data_type: 2,
                    _unknown0: 0
                },
                ConfigVarInfo {
                    section: "Visuals".into(),
                    name: "AllowClothSimulationOnGpu".into(),
                    value: "false".into(),
                    data_type: 1,
                    _unknown0: 0
                },
                ConfigVarInfo {
                    section: "Visuals".into(),
                    name: "HdrGamma1".into(),
                    value: "1.1".into(),
                    data_type: 3,
                    _unknown0: 0
                }
            ]
        }).encode_into(socket).unwrap();
    }
}
