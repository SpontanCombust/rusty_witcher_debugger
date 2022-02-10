use std::net::{TcpStream, Shutdown};
use std::io::{self, Write, BufRead};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::{thread, time};

use rw3d_core::{ constants, commands, packet::WitcherPacket };
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[clap(name="Rusty Witcher 3 Debugger", version="0.1")]
#[clap(about="A standalone debugging tool for The Witcher 3 written in Rust", long_about=None)]
struct Cli {
    /// IPv4 address of the machine on which the game is run
    #[clap(long, default_value="127.0.0.1")]
    ip: String,

    /// Option to disable messages sent from the game
    #[clap(long)]
    no_listen: bool,

    /// Option to enable verbose printing of packet contents
    #[clap(long)]
    verbose: bool,

    /// Command to use
    #[clap(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Get the root path to game scripts
    RootPath,
    /// Reload game scripts
    Reload,
    /// Run an exec function in the game
    Exec{ cmd: String },
}

fn main() {
    let cli = Cli::parse();

    let connection = try_connect(cli.ip.clone(), 5, 1000);

    match connection {
        Some(mut stream) => {
            println!("Successfully connected to the game!");

            small_delay();
            
            if !cli.no_listen {
                println!("Setting up listeners...");
                let listeners = commands::listen_all();
                for l in &listeners {
                    stream.write( l.to_bytes().as_slice() ).unwrap();
                }
            }

            small_delay();

            println!("Handling command...");
            let p = match &cli.command {
                CliCommands::Reload => {
                    commands::scripts_reload()
                }
                CliCommands::Exec { cmd } => {
                    commands::scripts_execute(&cmd)
                }
                CliCommands::RootPath => {
                    commands::scripts_root_path()
                }
            };
            stream.write( p.to_bytes().as_slice() ).unwrap();

            small_delay();

            let (snd, rcv) = std::sync::mpsc::channel();
            std::thread::spawn(move || reader_thread(stream, rcv, cli.verbose));

            // wait for any key in the main thread to exit
            pause();
            // terminate reader thread
            let _ = snd.send(());
            println!("Exiting...");
        }
        None => {
            println!("Failed to connect to the game on address {}", cli.ip);
        }
    }
}



fn try_connect(ip: String, max_tries: u8, tries_delay_ms: u64) -> Option<TcpStream> {
    let mut tries = max_tries;

    while tries > 0 {
        println!("Connecting to the game...");

        match TcpStream::connect(ip.clone() + ":" + constants::GAME_PORT) {
            Ok(conn) => {
                return Some(conn);
            }
            Err(e) => {
                println!("{}", e);
            }
        }

        tries -= 1;
        thread::sleep( time::Duration::from_millis(tries_delay_ms) );
    }

    None
}

fn small_delay() {
    for _ in 0..5 {
        thread::sleep( time::Duration::from_millis(100) );
    }
}

fn reader_thread(mut stream: TcpStream, cancel_token: Receiver<()>, verbose_print: bool ) {
    let mut peek_buffer = [0u8;6];
    let mut packet_available: bool;
    loop {
        // test if the thread has been ordered to stop
        match cancel_token.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        // Test if there are packets available to be read from stream
        match stream.peek(&mut peek_buffer) {
            Ok(size) => {
                packet_available = size > 0;
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }

        if packet_available {
            match WitcherPacket::from_stream(&mut stream) {
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
        } else {
            small_delay();
        }
    }

    if let Err(e) = stream.shutdown(Shutdown::Both) {
        println!("{}", e);
    }
}

fn pause() {
    let mut line = String::new();
    let stdin = io::stdin();

    println!("Press Enter to exit after you're done reading the output.\n");
    // read a single byte and discard
    let _ = stdin.lock().read_line(&mut line);
}