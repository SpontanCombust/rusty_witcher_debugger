use std::{thread, net::{Shutdown, TcpStream}, sync::mpsc::{Receiver, TryRecvError}, time::Duration, io::Write};

use clap::Subcommand;

use crate::{input_waiter::input_waiter, CliOptions};

/// Subcommands that require connection to game's socket and sending messages to it
#[derive(Subcommand)]
pub(crate) enum ServerSubcommands {
    /// Get the root path to game scripts
    Rootpath,
    /// Reload game scripts
    Reload,
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
    let connection = try_connect(options.ip.clone(), 5, 1000);
    
    match connection {
        Some(mut stream) => {
            if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
            println!("Successfully connected to the game!");

            if !options.no_listen {
                if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
                println!("Setting up listeners...");

                let listeners = rw3d_core::commands::listen_all();
                for l in &listeners {
                    stream.write( l.to_bytes().as_slice() ).unwrap();
                }
            }


            if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
            println!("Handling the command...");

            let p = match cmd {
                ServerSubcommands::Reload => {
                    rw3d_core::commands::scripts_reload()
                }
                ServerSubcommands::Exec { cmd } => {
                    rw3d_core::commands::scripts_execute(cmd)
                }
                ServerSubcommands::Rootpath => {
                    rw3d_core::commands::scripts_root_path()
                }
                ServerSubcommands::Modlist => {
                    rw3d_core::commands::mod_list()
                }
                ServerSubcommands::Opcode { func_name, class_name } => {
                    rw3d_core::commands::opcode(func_name, class_name)
                }
                ServerSubcommands::Varlist { section, name } => {
                    rw3d_core::commands::var_list(section, name)
                }
                // ServerSubcommands::Varset { section, name, value } => {
                //     rw3d_core::commands::var_set(section, name, value)
                // }
            };

            stream.write( p.to_bytes().as_slice() ).unwrap();


            if !options.no_wait || !options.no_listen { 
                println!("\nYou can press Enter at any moment to exit the program.\n");
                if !options.no_wait { thread::sleep( Duration::from_millis(3000) ) }
            }

            if !options.no_listen {
                println!("Game response:\n");
                if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
    
                // Channel to communicate to and from the the reader
                let (reader_snd, reader_rcv) = std::sync::mpsc::channel();
    
                // This thread is not expected to finish, so we won't assign a handle to it
                // Takes reader_snd so it can communicate to the reader thread to stop execution when user presses Enter
                std::thread::spawn(move || input_waiter(reader_snd) );
    
                // This function can either finish by itself by the means of response timeout
                // or be stopped by input waiter thread if that one sends him a signal
                read_responses(&mut stream, options.response_timeout, reader_rcv, options.verbose);

            } else {
                // Wait a little bit to not finish the connection abruptly
                thread::sleep( Duration::from_millis(500) );        
            }

            if let Err(e) = stream.shutdown(Shutdown::Both) {
                println!("{}", e);
            }

        }
        None => {
            println!("Failed to connect to the game on address {}", options.ip);
        }
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

fn read_responses(stream: &mut TcpStream, response_timeout: i64, cancel_token: Receiver<()>, verbose_print: bool ) {
    let mut peek_buffer = [0u8;6];
    let mut packet_available: bool;
    let mut response_wait_elapsed: i64 = 0;

    const READ_TIMEOUT: i64 = 1000;
    // Timeout is set so that the peek operation won't block the thread indefinitely after it runs out of data to read
    stream.set_read_timeout( Some(Duration::from_millis(READ_TIMEOUT as u64)) ).unwrap();

    loop {
        // test if the thread has been ordered to stop
        match cancel_token.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

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
                    if verbose_print {
                        println!("{:?}", packet);
                    } else {
                        println!("{}", packet);
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    break;
                }
            }

            response_wait_elapsed = 0;

        } else {
            // if not available it means peek probably waited TIMEOUT millis before it returned
            response_wait_elapsed += READ_TIMEOUT;

            if response_timeout >= 0 && response_wait_elapsed >= response_timeout {
                println!("\nGame response timeout reached.");
                break;
            }
        }
    }
}