use colored::Colorize;
use rw3d_core::packet::WitcherPacket;
use rw3d_core::utils::{scripts_execute_formatter, scripts_root_path_formatter, mod_list_formatter, opcode_formatter, var_list_formatter, scripts_reload_formatter, scripts_reload_response_type, ScriptsReloadResponseType};



pub(crate) trait HandleResponse {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool);
    fn is_done(&self) -> bool;
}



pub(crate) struct ScriptsReloadPrinter {
    has_finished: bool,
    warnings: Vec<String>,
    errors: Vec<String>
}

impl ScriptsReloadPrinter {
    fn print_summary(&self) {
        println!("========== {} Errors, {} Warnings ==========\n", self.errors.len(), self.warnings.len());

        for e in &self.errors {
            println!("{}", e.red());
        }

        println!(""); // empty line between errors and warnings

        for w in &self.warnings {
            println!("{}", w.yellow());
        }
    }
}

impl Default for ScriptsReloadPrinter {
    fn default() -> Self {
        ScriptsReloadPrinter {
            has_finished: false,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl HandleResponse for ScriptsReloadPrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        let msg = scripts_reload_formatter(&response);

        if let Ok(response_type) = scripts_reload_response_type(&response) {
            match response_type {
                ScriptsReloadResponseType::Started => {
                    self.has_finished = false;
                }
                ScriptsReloadResponseType::Warn {..} => {
                    self.warnings.push(msg.to_string());
                }
                ScriptsReloadResponseType::Error {..} => {
                    self.errors.push(msg.to_string());
                }
                ScriptsReloadResponseType::Finished(_) => {
                    self.has_finished = true;
                }
                _ => {}
            }
        }

        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", msg);
        }

        if self.has_finished && ( !self.warnings.is_empty() || !self.errors.is_empty() ) {
            self.print_summary();
        }
    }

    fn is_done(&self) -> bool {
        self.has_finished
    }
}



pub(crate) struct ScriptsExecutePrinter();

impl HandleResponse for ScriptsExecutePrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", scripts_execute_formatter(&response));
        }
    }

    fn is_done(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct ScriptsRootpathPrinter();

impl HandleResponse for ScriptsRootpathPrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", scripts_root_path_formatter(&response));
        }
    }

    fn is_done(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct ModlistPrinter();

impl HandleResponse for ModlistPrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", mod_list_formatter(&response));
        }
    }

    fn is_done(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct OpcodePrinter();

impl HandleResponse for OpcodePrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", opcode_formatter(&response));
        }
    }

    fn is_done(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct VarlistPrinter();

impl HandleResponse for VarlistPrinter {
    fn handle_response(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", var_list_formatter(&response));
        }
    }

    fn is_done(&self) -> bool {
        true // only one packet
    }
}