use rw3d_core::packet::WitcherPacket;
use rw3d_core::utils::{scripts_execute_formatter, scripts_root_path_formatter, mod_list_formatter, opcode_formatter, var_list_formatter, scripts_reload_formatter, scripts_reload_response_type, ScriptsReloadResponseType};



pub(crate) trait HandleResponse {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool);
    fn should_exit(&self) -> bool;
}



pub(crate) struct ScriptsReloadHandler {
    has_finished: bool,
    warnings: Vec<String>,
    errors: Vec<String>
}

impl ScriptsReloadHandler {
    fn print_summary(&self) {
        println!("{} Errors, {} Warnings\n", self.errors.len(), self.warnings.len());

        for e in &self.errors {
            println!("{}", e);
        }

        println!(""); // empty line between errors and warnings

        for w in &self.warnings {
            println!("{}", w);
        }
    }
}

impl Default for ScriptsReloadHandler {
    fn default() -> Self {
        ScriptsReloadHandler {
            has_finished: false,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl HandleResponse for ScriptsReloadHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
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

    fn should_exit(&self) -> bool {
        self.has_finished
    }
}



pub(crate) struct ScriptsExecuteHandler();

impl HandleResponse for ScriptsExecuteHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", scripts_execute_formatter(&response));
        }
    }

    fn should_exit(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct ScriptsRootpathHandler();

impl HandleResponse for ScriptsRootpathHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", scripts_root_path_formatter(&response));
        }
    }

    fn should_exit(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct ModlistHandler();

impl HandleResponse for ModlistHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", mod_list_formatter(&response));
        }
    }

    fn should_exit(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct OpcodeHandler();

impl HandleResponse for OpcodeHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", opcode_formatter(&response));
        }
    }

    fn should_exit(&self) -> bool {
        true // only one packet
    }
}



pub(crate) struct VarlistHandler();

impl HandleResponse for VarlistHandler {
    fn handle(&mut self, response: WitcherPacket, verbose_print: bool) {
        if verbose_print {
            println!("{:?}", response);
        } else {
            println!("{}", var_list_formatter(&response));
        }
    }

    fn should_exit(&self) -> bool {
        true // only one packet
    }
}