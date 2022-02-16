use std::{thread, time::Duration};

use clap::Subcommand;

use crate::{input_waiter::input_waiter, CliOptions};

/// Subcommands that can be executed without connecting to game's socket
#[derive(Subcommand)]
pub(crate) enum LocalSubcommands {
    /// Prints game's script logs onto console
    Scriptslog
}

pub(crate) fn handle_local_subcommand( cmd: LocalSubcommands, options: CliOptions ) {
    if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
    println!("Handling the command...");

    let (logger_snd, logger_rcv) = std::sync::mpsc::channel();

    std::thread::spawn(move || input_waiter(logger_snd) );

    println!("\nYou can press Enter at any moment to exit the program.\n");
    if !options.no_wait { thread::sleep( Duration::from_millis(3000) ) }

    match cmd {
        LocalSubcommands::Scriptslog => {
            if let Some(err) = rw3d_core::scriptslog::read_from_scriptslog(|s| print!("{}", s), 1000, logger_rcv) {
                println!("{}", err);
            }
        }
    }
}