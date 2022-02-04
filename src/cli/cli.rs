use std::net::{TcpStream};
use std::io::{Read, Write};
use std::{thread, time};
use rusty_witcher_debugger_core::{ constants, commands, packet::WitcherPacket };

fn main() {
    let mut connection : Option<TcpStream> = None;
    let mut tries = 5;
    while tries > 0 {
        println!("Connecting to the game...");

        match TcpStream::connect("127.0.0.1:".to_owned() + constants::GAME_PORT) {
            Ok(conn) => {
                connection = Some(conn);
                break;
            }
            Err(_) => ()
        }

        tries -= 1;
        thread::sleep( time::Duration::from_millis(500) );
    }

    match connection {
        Some(mut stream) => {
            println!("Successfully connected to the game!");

            let bind_cmd = commands::bind("scripts");
            stream.write( bind_cmd.to_bytes().as_slice() ).unwrap();

            let mut buffer = Vec::<u8>::with_capacity( bind_cmd.size() );
            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    // match WitcherPacket::from_bytes(&buffer) {
                    //     Ok(packet) => {
                    //         if packet == bind_cmd {
                    //             println!("Echo performed successfully!");
                    //         } else {
                    //             println!("Echo failed!");
                    //         }
                    //     }
                    //     Err(e) => {
                    //         println!("Failed to parse packet: {}", e);
                    //     }
                    // }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        None => {
            println!("Failed to connect to the game!");
        }
    }
}