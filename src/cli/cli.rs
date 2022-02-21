mod server_subcommands;
mod local_subcommands;
mod input_waiter;
mod response_handling;

use local_subcommands::{LocalSubcommands, handle_local_subcommand};
use server_subcommands::{ServerSubcommands, handle_server_subcommand};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[clap(name="Rusty Witcher 3 Debugger", version="0.4")]
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

    /// Exit the program almost immediately after executing the command without listening to responses coming from the game.
    /// Doesn't apply to scriptslog command.
    #[clap(long)]
    no_listen: bool,

    /// Enable verbose printing of packet contents.
    #[clap(long)]
    verbose: bool,

    /// Execute command immediately without doing short breaks between info messages beforehand.
    #[clap(long)]
    no_wait: bool,

    /// The maximum amount of milliseconds that program should wait for any game messages until it will automatically exit.
    /// This setting is ignored if --no-listen is set.
    /// If set to a negative number will wait indefinitely for user's input.
    /// Doesn't apply to scriptslog command.
    #[clap(long, short, default_value_t=-1)]
    response_timeout: i64,
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


fn main() {
    let cli = Cli::parse();

    match cli.command {
        CliCommands::ServerSubcommands(c) => handle_server_subcommand(c, cli.options),
        CliCommands::LocalSubcommands(c) => handle_local_subcommand(c, cli.options),
    }
}
