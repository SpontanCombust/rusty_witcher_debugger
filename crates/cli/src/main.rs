mod server_subcommands;
mod local_subcommands;
mod response_handling;
mod logging;

use local_subcommands::{LocalSubcommands, handle_local_subcommand};
use logging::LOG_LEVEL;
use server_subcommands::{ServerSubcommands, handle_server_subcommand};
use clap::{ArgEnum, Parser, Subcommand};


#[derive(Parser)]
#[clap(name="Rusty Witcher 3 Debugger", version=env!("CARGO_PKG_VERSION"))]
#[clap(about="A standalone debugging tool for The Witcher 3 written in Rust", long_about=None)]
struct Cli {
    #[clap(flatten)]
    options: CliOptions,    

    #[clap(subcommand)]
    command: CliCommands,
}

#[derive(Parser)]
pub(crate) struct CliOptions {
    /// IPv4 address of the machine on which the game is run.
    #[clap(long, default_value="127.0.0.1")]
    ip: String,

    /// Specify what logs are allowed to be printed to the standard output.
    /// Does not apply to output from the `scriptslog` command.
    #[clap(long, short='l', value_enum, default_value="all")]
    log_level: LogLevel, 

    /// Enable verbose printing of packet contents.
    #[clap(long, short='v')]
    verbose: bool,

    /// Execute command immediately without doing short breaks between info messages beforehand.
    #[clap(long)]
    no_delay: bool, 

    /// The maximum amount of milliseconds that program should wait for the game to respond.
    /// It will also affect how quickly the program shuts down.
    #[clap(long, short='t', default_value_t=2000)] 
    response_timeout: u64,
}

#[derive(Debug, ArgEnum, Clone, Copy, PartialEq, Eq)]
enum LogLevel {
    /// Print all messages.
    All,
    /// Print only result of the command.
    OutputOnly,
    /// Don't print anything. This will effectively make the programm exit immediately after executing the command.
    None
}

#[derive(Subcommand)]
enum CliCommands {
    /// Subcommands that require connection to game's socket and sending messages to it
    #[clap(flatten)]
    ServerSubcommands(ServerSubcommands),

    /// Subcommands that can be executed without connecting to game's socket
    #[clap(flatten)]
    LocalSubcommands(LocalSubcommands)
}


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    LOG_LEVEL.set(cli.options.log_level).unwrap();

    match cli.command {
        CliCommands::ServerSubcommands(c) => handle_server_subcommand(c, cli.options)?,
        CliCommands::LocalSubcommands(c) => handle_local_subcommand(c, cli.options),
    }

    Ok(())
}
