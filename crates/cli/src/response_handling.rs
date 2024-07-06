use std::sync::mpsc::Sender;

use colored::Colorize;
use rw3d_net::{messages::{notifications::*, requests::*}, protocol::WitcherPacket};

use crate::logging::println_output;


pub fn print_raw_packet(packet: WitcherPacket) {
    println_output(format!("{:?}", packet));
}


pub struct ScriptsReloadPrinter {
    warnings: Vec<String>,
    errors: Vec<String>,
    finished_token: Sender<()>,
    verbose_printing: bool
}

impl ScriptsReloadPrinter {
    pub fn new(finished_token: Sender<()>, verbose_printing: bool) -> Self {
        ScriptsReloadPrinter {
            warnings: Vec::new(),
            errors: Vec::new(),
            finished_token,
            verbose_printing
        }
    }

    pub fn print_progress(&mut self, params: ScriptsReloadProgressParams) {
        match params {
            ScriptsReloadProgressParams::Started => if !self.verbose_printing {
                println_output("Script compilation started...");
            }
            ScriptsReloadProgressParams::Log { message } if !self.verbose_printing => {
                println_output(message)
            }
            ScriptsReloadProgressParams::Warn { line, local_script_path, message } if !self.verbose_printing => {
                let s = format!("[Warning] {}({}): {}", local_script_path.display(), line, message);
                println_output(&s);
                self.warnings.push(s);
            }
            ScriptsReloadProgressParams::Error { line, local_script_path, message } if !self.verbose_printing => {
                let s = format!("[Error] {}({}): {}", local_script_path.display(), line, message);
                println_output(&s);
                self.errors.push(s);
            }
            ScriptsReloadProgressParams::Finished { success } => {
                if !self.verbose_printing {
                    if success {
                        println_output("Script compilation finished successfully.");
                    } else {
                        println_output("Script compilation finished with errors.");
                    }
    
                    if !self.warnings.is_empty() || !self.errors.is_empty() {
                        println_output("");
                        self.print_summary();
                    }
                }

                let _ = self.finished_token.send(());
            },
            _ => {}
        }
    }

    fn print_summary(&self) {
        println_output(format!("========== {} Errors, {} Warnings ==========", self.errors.len(), self.warnings.len()));

        for e in &self.errors {
            println_output(e.red());
        }

        if self.errors.len() > 0 {
            println_output(""); // empty line between errors and warnings
        }

        for w in &self.warnings {
            println_output(w.yellow());
        }
    }
}


pub fn print_exec_result(result: ExecuteCommandResult) {
    match result {
        ExecuteCommandResult::Success { log_output } => {
            if let Some(log_output) = log_output {
                println_output(log_output.join("\n"))
            } else {
                println_output("Command executed successfully")
            }
        }
        ExecuteCommandResult::Fail => {
            println_output("Command failed to execute")
        }
    }
}


pub fn print_root_path_result(result: ScriptsRootPathResult) {
    println_output(result.abs_path.display())
}


pub fn print_mod_list_result(result: ScriptPackagesResult) {
    println_output(format!("Mods installed: {}", result.packages.len() - 1)); // one is always content0

    let mut mods = result.packages.into_iter()
        .filter(|p| p.package_name != "content0")
        .map(|p| p.package_name)
        .collect::<Vec<_>>();

    mods.sort();
    for m in mods {
        println_output(m);
    }
}


pub fn print_opcodes(result: OpcodesResult) {
    for breakdown in result.breakdowns {
        println_output(format!("Line {}", breakdown.line));
        println_output("Opcodes: ");
        for opcode in breakdown.opcodes {
            println_output(opcode);
        }
    }
}


pub fn print_var_list(result: ConfigVarsResult) {
    let tab_line = format!("{}+-{}+-{}", "-".repeat(40), "-".repeat(45), "-".repeat(40) );
    println_output(&tab_line);
    println_output(format!("{:40}| {:45}| {}", "Section", "Variable", "Value"));
    println_output(&tab_line);

    let mut vars = result.vars;
    vars.sort_by(|v1, v2| {
        match v1.section.cmp(&v2.section) {
            std::cmp::Ordering::Equal => v1.name.cmp(&v2.name),
            other => other
        }
    });

    for var in vars {
        println_output(format!("{:40}| {:45}| {}", var.section, var.name, var.value));
    }
}
