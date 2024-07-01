use std::{thread, time::Duration};

use clap::{ Parser, Subcommand};
use colored::{Colorize, Color};

use crate::CliOptions;


/// Subcommands that can be executed without connecting to game's socket
#[derive(Subcommand)]
pub(crate) enum LocalSubcommands {
    /// Prints game's script logs onto console
    Scriptslog {
        /// How often should the log be refreshed, in millis
        #[clap(short='t', long, default_value_t=1000, display_order=0)]
        refresh_time: u64,

        /// Filter out lines that do not containt highlighted text
        #[clap(short='f', long, display_order=1)]
        filter_non_highlighted: bool,

        /// Specify a custom, full path to the scriptslog file
        #[clap(short='p', long, display_order=2)]
        custom_path: Option<String>,

        /// Flags for setting highlight colors for lines that contain a certain string
        /// Colors inside mean the color of the background of the highlighted line
        #[clap(flatten)]
        colors: ScriptslogColors,
    }
}

#[derive(Parser)]
pub(crate) struct ScriptslogColors {
    /// Highlight lines containing specified text in black
    #[clap(long, value_name="TEXT", display_order=3)]
    black: Vec<String>,
    /// Highlight lines containing specified text in red
    #[clap(long, value_name="TEXT", display_order=4)]
    red: Vec<String>,
    /// Highlight lines containing specified text in green
    #[clap(long, value_name="TEXT", display_order=5)]
    green: Vec<String>,
    /// Highlight lines containing specified text in yellow
    #[clap(long, value_name="TEXT", display_order=6)]
    yellow: Vec<String>,
    /// Highlight lines containing specified text in blue
    #[clap(long, value_name="TEXT", display_order=7)]
    blue: Vec<String>,
    /// Highlight lines containing specified text in magenta
    #[clap(long, value_name="TEXT", display_order=8)]
    magenta: Vec<String>,
    /// Highlight lines containing specified text in cyan
    #[clap(long, value_name="TEXT", display_order=9)]
    cyan: Vec<String>,
    /// Highlight lines containing specified text in white
    #[clap(long, value_name="TEXT", display_order=10)]
    white: Vec<String>,
}

pub(crate) fn handle_local_subcommand( cmd: LocalSubcommands, options: CliOptions ) {
    println!("Executing the command...");
    if !options.no_delay { thread::sleep( Duration::from_millis(500)) }

    match cmd {
        LocalSubcommands::Scriptslog { colors, refresh_time, filter_non_highlighted, custom_path } => {
            println!("\nYou can press Ctrl-C at any moment to exit the program.\n");
            if !options.no_delay { thread::sleep( Duration::from_millis(1000)) }

            let (cancel_sender, cancel_token) = std::sync::mpsc::channel();
            ctrlc::set_handler(move || cancel_sender.send(()).expect("Failed to send cancel signal")).expect("Failed to set Ctrl-C handler");

            let highlights = scriptslog_colors_to_highlight_records(colors);
            if let Some(err) = rw3d_scriptslog::tail_scriptslog(|text| scriptslog_printer(text, &highlights, filter_non_highlighted), refresh_time, cancel_token, custom_path) {
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