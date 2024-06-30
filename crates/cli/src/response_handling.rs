use colored::Colorize;
use rw3d_net::{messages::{notifications::*, requests::*}, protocol::WitcherPacket};


pub fn print_raw_packet(packet: WitcherPacket) {
    println!("{:?}", packet);
}


pub struct ScriptsReloadPrinter {
    warnings: Vec<String>,
    errors: Vec<String>
}

impl ScriptsReloadPrinter {
    pub fn new() -> Self {
        ScriptsReloadPrinter {
            warnings: Vec::new(),
            errors: Vec::new()
        }
    }

    pub fn print_progress(&mut self, params: ScriptsReloadProgressParams) {
        match params {
            ScriptsReloadProgressParams::Started => {
                println!("Script compilation started...");
            }
            ScriptsReloadProgressParams::Log { message } => {
                println!("{}", message)
            }
            ScriptsReloadProgressParams::Warn { line, local_script_path, message } => {
                println!("[Warning] {}({}): {}", local_script_path.display(), line, message )
            }
            ScriptsReloadProgressParams::Error { line, local_script_path, message } => {
                println!("[Error] {}({}): {}", local_script_path.display(), line, message )
            }
            ScriptsReloadProgressParams::Finished { success } => {
                if success {
                    println!("Script compilation finished successfully.");
                } else {
                    println!("Script compilation finished with errors.");
                }

                if !self.warnings.is_empty() || !self.errors.is_empty() {
                    println!();
                    self.print_summary();
                }
            }
        }
    }

    fn print_summary(&self) {
        println!("========== {} Errors, {} Warnings ==========", self.errors.len(), self.warnings.len());

        for e in &self.errors {
            println!("{}", e.red());
        }

        if self.errors.len() > 0 {
            println!(); // empty line between errors and warnings
        }

        for w in &self.warnings {
            println!("{}", w.yellow());
        }
    }
}


pub fn print_exec_result(result: ExecuteCommandResult) {
    match result {
        ExecuteCommandResult::Success { log_output } => {
            if let Some(log_output) = log_output {
                println!("{}", log_output.join("\n"))
            } else {
                println!("Command executed successfully")
            }
        }
        ExecuteCommandResult::Fail => {
            println!("Command failed to execute")
        }
    }
}


pub fn print_root_path_result(result: ScriptsRootPathResult) {
    println!("{}", result.abs_path.display())
}


pub fn print_mod_list_result(result: ScriptPackagesResult) {
    println!("Mods installed: {}", result.packages.len() - 1); // one is always content0

    let mut mods = result.packages.into_iter()
        .filter(|p| p.package_name != "content0")
        .map(|p| p.package_name)
        .collect::<Vec<_>>();

    mods.sort();
    for m in mods {
        println!("{}", m);
    }
}


pub fn print_opcodes(result: OpcodesResult) {
    for breakdown in result.breakdowns {
        println!("Line {}", breakdown.line);
        println!("Opcodes: ");
        for opcode in breakdown.opcodes {
            println!("{}", opcode);
        }
    }
}


pub fn print_var_list(result: ConfigVarsResult) {
    let tab_line = format!("{}+-{}+-{}", "-".repeat(40), "-".repeat(45), "-".repeat(40) );
    println!("{}", tab_line);
    println!("{:40}| {:45}| {}", "Section", "Variable", "Value");
    println!("{}", tab_line);

    let mut vars = result.vars;
    vars.sort_by(|v1, v2| {
        match v1.section.cmp(&v2.section) {
            std::cmp::Ordering::Equal => v1.name.cmp(&v2.name),
            other => other
        }
    });

    for var in vars {
        println!("{:40}| {:45}| {}", var.section, var.name, var.value);
    }
}
