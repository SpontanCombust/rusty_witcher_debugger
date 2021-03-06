use std::{thread, net::{Shutdown, TcpStream}, time::Duration, io::Write};

use clap::Subcommand;

use crate::{CliOptions, response_handling::{HandleResponse, ScriptsReloadPrinter, ScriptsExecutePrinter, ScriptsRootpathPrinter, ModlistPrinter, OpcodePrinter, VarlistPrinter}};

/// Subcommands that require connection to game's socket and sending messages to it
#[derive(Subcommand)]
pub(crate) enum ServerSubcommands {
    /// Get the root path to game scripts
    Rootpath,
    /// Reload game scripts
    Reload {
        // Max waiting time for function compilation in millis
        #[clap(long, short='c', default_value_t=7000)]
        max_compile_time: u64
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
    //FIXME not working, probably incorrect packet format
    // /// Sets a config variable
    // Varset {
    //     /// Variable's section
    //     #[clap(short)]
    //     section: String,
    //     /// Variable's name
    //     #[clap(short)]
    //     name: String,
    //     /// Variable's new value
    //     #[clap(short)]
    //     value: String
    // },
}


pub(crate) fn handle_server_subcommand( cmd: ServerSubcommands, options: CliOptions ) {

    if let Some(mut connection) = try_connect(options.ip.clone(), 5, 1000) {

        if !options.no_wait { thread::sleep( Duration::from_millis(500) ) }
        println!("Successfully connected to the game!");

        if !options.no_listen {
            if !options.no_wait { thread::sleep( Duration::from_millis(500) ) }
            println!("Setting up listeners...");

            let listeners = rw3d_core::commands::listen_all();
            for l in &listeners {
                connection.write( l.to_bytes().as_slice() ).unwrap();
            }
        }


        if !options.no_wait { thread::sleep( Duration::from_millis(500) ) }
        println!("Handling the command...");

        let response_handler: Box<dyn HandleResponse>;
        let p = match cmd {
            ServerSubcommands::Reload { max_compile_time } => {
                response_handler = Box::new(ScriptsReloadPrinter::new(max_compile_time));
                rw3d_core::commands::scripts_reload()
            }
            ServerSubcommands::Exec { cmd } => {
                response_handler = Box::new(ScriptsExecutePrinter());
                rw3d_core::commands::scripts_execute(cmd)
            }
            ServerSubcommands::Rootpath => {
                response_handler = Box::new(ScriptsRootpathPrinter());
                rw3d_core::commands::scripts_root_path()
            }
            ServerSubcommands::Modlist => {
                response_handler = Box::new(ModlistPrinter());
                rw3d_core::commands::mod_list()
            }
            ServerSubcommands::Opcode { func_name, class_name } => {
                response_handler = Box::new(OpcodePrinter());
                rw3d_core::commands::opcode(func_name, class_name)
            }
            ServerSubcommands::Varlist { section, name } => {
                response_handler = Box::new(VarlistPrinter());
                rw3d_core::commands::var_list(section, name)
            }
            // ServerSubcommands::Varset { section, name, value } => {
            //     rw3d_core::commands::var_set(section, name, value)
            // }
        };

        connection.write( p.to_bytes().as_slice() ).unwrap();

        if !options.no_listen {
            println!("\nGame response:\n");
            if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }

            // This function can either finish by itself by the means of response timeout
            // or be stopped by input waiter thread if that one sends him a signal
            read_responses(&mut connection, options.response_timeout, options.verbose, response_handler);

        } else {
            // Wait a little bit to not finish the connection abruptly
            thread::sleep( Duration::from_millis(500) );        
        }

        if let Err(e) = connection.shutdown(Shutdown::Both) {
            println!("{}", e);
        }

    } else {
        println!("Failed to connect to the game on address {}", options.ip);
        println!("Make sure the game is running and that it was launched with following flags: -net -debugscripts.");
    } 
}

fn try_connect(ip: String, max_tries: u8, tries_delay_ms: u64) -> Option<TcpStream> {
    let mut tries = max_tries;

    while tries > 0 {
        println!("Connecting to the game...");

        match TcpStream::connect(ip.clone() + ":" + rw3d_core::constants::GAME_PORT) {
            Ok(conn) => {
                return Some(conn);
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        tries -= 1;
        thread::sleep( Duration::from_millis(tries_delay_ms) );
    }

    None
}

fn read_responses(stream: &mut TcpStream, response_timeout: u64, verbose_print: bool, mut handler: Box<dyn HandleResponse>) {
    let mut peek_buffer = [0u8;6];
    let mut packet_available: bool;
    let mut response_wait_elapsed: u64 = 0;

    const READ_TIMEOUT: u64 = 1000;
    // Timeout is set so that the peek operation won't block the thread indefinitely after it runs out of data to read
    stream.set_read_timeout( Some(Duration::from_millis(READ_TIMEOUT)) ).unwrap();

    loop {
        // Test if there are packets available to be read from stream
        // This can block up to the amount specified with set_read_timeout
        match stream.peek(&mut peek_buffer) {
            Ok(size) => {
                packet_available = size > 0;
            }
            Err(_) => {
                packet_available = false;
            }
        }

        if packet_available {
            match rw3d_core::packet::WitcherPacket::from_stream(stream) {
                Ok(packet) => {
                    handler.handle_response(packet, verbose_print); 
                }
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }

            if handler.is_done() {
                break;
            }

            response_wait_elapsed = 0;

        } else {
            // if not available it means peek probably waited READ_TIMEOUT millis before it returned
            response_wait_elapsed += READ_TIMEOUT;            
        }

        if response_wait_elapsed >= handler.response_await_time() + response_timeout {
            println!("\nGame response timeout reached.");
            break;
        }
    }
}