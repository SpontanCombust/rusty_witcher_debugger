use std::{net::Ipv4Addr, str::FromStr, thread, time::Duration};

use anyhow::Context;
use clap::Subcommand;
use rw3d_net::{connection::WitcherConnection, messages::requests::*};
use rw3d_net_client::WitcherClient;

use crate::{CliOptions, response_handling::*};


/// Subcommands that require connection to game's socket and sending messages to it
#[derive(Subcommand)]
pub(crate) enum ServerSubcommands {
    /// Get the root path to game scripts
    Rootpath,
    /// Reload game scripts
    Reload {
        /// Max waiting time for function compilation in milliseconds
        #[clap(long, short='t')]
        max_compile_time: Option<u64>
    },
    /// Run an exec function in the game
    Exec{
        /// Command to be run in the game
        cmd: String 
    },
    /// Get the list of mods installed
    Modlist,
    /// Get opcode of a script function
    Opcode {
        /// Name of the function
        #[clap(short)]
        func_name: String, 
        /// Name of the class; can be empty
        #[clap(short)]
        class_name: Option<String> 
    },
    /// Search for config variables
    Varlist {
        /// Var section to search; if left empty searches all sections
        #[clap(short)]
        section: Option<String>,
        /// Token that should be included in vars; if left empty searches all variables
        #[clap(short)]
        name: Option<String>
    },
}


pub(crate) fn handle_server_subcommand( cmd: ServerSubcommands, options: CliOptions ) -> anyhow::Result<()> {
    let ip = Ipv4Addr::from_str(&options.ip).context("Invalid IPv4 address specified")?;

    let mut connection = try_connect(ip)?;
    connection.set_read_timeout(Some(Duration::from_millis(options.response_timeout))).unwrap();

    let client = WitcherClient::new(connection);
    client.start().context("Failed to start up the client")?;

    println!("Successfully connected to the game!");

    if !options.no_listen {
        println!("Setting up listeners...");

        client.listen_to_all_namespaces().context("Failed to set up listeners")?;
    }


    if options.verbose {
        client.on_raw_packet(print_raw_packet);
    }

    println!("Executing the command...\n");
    if !options.no_delay { thread::sleep( Duration::from_millis(500) ) }

    match cmd {
        ServerSubcommands::Reload { max_compile_time } => {
            let (finished_token, did_finish) = std::sync::mpsc::channel();
            let mut scripts_reload_printer = ScriptsReloadPrinter::new(finished_token, options.verbose);
            client.on_scripts_reload_progress(move |params| {
                scripts_reload_printer.print_progress(params);
            });

            client.reload_scripts()?;

            if let Some(max_compile_time) = max_compile_time {
                if let Err(_) = did_finish.recv_timeout(std::time::Duration::from_millis(max_compile_time)) {
                    println!("Scripts didn't compile in the specified time. Exiting early...");
                }
            } else {
                did_finish.recv()?
            }
        }
        ServerSubcommands::Exec { cmd } => {
            let result = client.execute_command(ExecuteCommandParams {
                cmd
            })?;

            // If printing is verbose it is handled by a notification callback
            if !options.verbose {
                print_exec_result(result);
            }
        }
        ServerSubcommands::Rootpath => {
            let result = client.scripts_root_path()?;

            if !options.verbose {
                print_root_path_result(result);
            }
        }
        ServerSubcommands::Modlist => {
            let result = client.script_packages()?;

            if !options.verbose {
                print_mod_list_result(result);
            }
        }
        ServerSubcommands::Opcode { func_name, class_name } => {
            let result = client.opcodes(OpcodesParams {
                class_name,
                func_name
            })?;

            if !options.verbose {
                print_opcodes(result);
            }
        }
        ServerSubcommands::Varlist { section, name } => {
            let result = client.config_vars(ConfigVarsParams {
                section_filter: section,
                name_filter: name
            })?;

            if !options.verbose {
                print_var_list(result);
            }
        }
    };

    println!("\nShutting down client...");
    client.stop().context("Failed to shut down client connection")
}

fn try_connect(ip: Ipv4Addr) -> anyhow::Result<WitcherConnection> {
    const TIMEOUT_MILLIS: u64 = 5000; 

    println!("Connecting to the game...");
    let err = format!("Failed to connect to the game on address {}.\n\
                       Make sure the game is running and that it was launched with following flags: -net -debugscripts.", ip.to_string());

    WitcherConnection::connect_timeout(ip.into(), Duration::from_millis(TIMEOUT_MILLIS)).context(err)
}
