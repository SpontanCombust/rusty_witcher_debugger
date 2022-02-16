use std::{thread, time::Duration};

use clap::{ Parser, Subcommand};
use colored::{Colorize, Color};

use crate::{input_waiter::input_waiter, CliOptions};

/// Subcommands that can be executed without connecting to game's socket
#[derive(Subcommand)]
pub(crate) enum LocalSubcommands {
    /// Prints game's script logs onto console
    Scriptslog {
        /// Flags for setting highlight colors for lines that contain a certain string
        /// Colors inside mean the color of the background of the highlighted line
        #[clap(flatten)]
        colors: ScriptslogColors,

        /// How often should the log be refreshed, in millis
        #[clap(short, long, default_value_t=1000)]
        refresh_time: u64,

        /// Filter out lines that do not containt highlighted text
        #[clap(short, long)]
        filter_non_highlighted: bool
    }
}

#[derive(Parser)]
pub(crate) struct ScriptslogColors {
    #[clap(long)]
    black: Vec<String>,
    #[clap(long)]
    red: Vec<String>,
    #[clap(long)]
    green: Vec<String>,
    #[clap(long)]
    yellow: Vec<String>,
    #[clap(long)]
    blue: Vec<String>,
    #[clap(long)]
    magenta: Vec<String>,
    #[clap(long)]
    cyan: Vec<String>,
    #[clap(long)]
    white: Vec<String>,
}

pub(crate) fn handle_local_subcommand( cmd: LocalSubcommands, options: CliOptions ) {
    if !options.no_wait { thread::sleep( Duration::from_millis(1000) ) }
    println!("Handling the command...");

    let (logger_snd, logger_rcv) = std::sync::mpsc::channel();

    std::thread::spawn(move || input_waiter(logger_snd) );

    println!("\nYou can press Enter at any moment to exit the program.\n");
    if !options.no_wait { thread::sleep( Duration::from_millis(3000) ) }

    match cmd {
        LocalSubcommands::Scriptslog { colors, refresh_time, filter_non_highlighted } => {
            let highlights = scriptslog_colors_to_highlight_records(colors);
            if let Some(err) = rw3d_core::scriptslog::tail_scriptslog(|text| scriptslog_printer(text, &highlights, filter_non_highlighted), refresh_time, logger_rcv) {
                println!("{}", err);
            }
        }
    }
}


struct ScriptslogHighlightRecord {
    pattern: String,
    fg: Color,
    bg: Color
}

fn scriptslog_colors_to_highlight_records(colors: ScriptslogColors) -> Vec<ScriptslogHighlightRecord> {
    let mut colored = Vec::new();

    for p in colors.black {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::White,
            bg: Color::Black
        });
    }
    for p in colors.red {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::White,
            bg: Color::Red
        });
    }
    for p in colors.green {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::Black,
            bg: Color::Green
        });
    }
    for p in colors.yellow {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::Black,
            bg: Color::Yellow
        });
    }
    for p in colors.blue {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::White,
            bg: Color::Blue
        });
    }
    for p in colors.magenta {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::White,
            bg: Color::Magenta
        });
    }
    for p in colors.cyan {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::Black,
            bg: Color::Cyan
        });
    }
    for p in colors.white {
        colored.push(ScriptslogHighlightRecord{
            pattern: p,
            fg: Color::Black,
            bg: Color::White
        });
    }

    colored
}

// Just a constant to initialize coloring variables and to signal to not apply any highlighting if this color is detected
// The color chosen is not used anywhere else for that matter
const NO_COLORING_PLACEHOLDER: Color = Color::BrightBlack;

fn scriptslog_printer( text: &String, highlights: &Vec<ScriptslogHighlightRecord>, filter_non_highlighted: bool ) {
    let lines = text.split("\n");

    for line in lines {
        let (mut fg, mut bg) = (NO_COLORING_PLACEHOLDER, NO_COLORING_PLACEHOLDER); 

        for h in highlights {
            if line.contains(&h.pattern) {
                fg = h.fg;
                bg = h.bg;
                break;
            }
        }

        if fg != NO_COLORING_PLACEHOLDER {
            println!("{}", line.color(fg).on_color(bg));
        } else if !filter_non_highlighted {
            println!("{}", line);
        }
    }
}