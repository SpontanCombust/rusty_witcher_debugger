use crate::{packet::WitcherPacket, packet_data::WitcherPacketData};


pub type ResponseFormatter = fn(&WitcherPacket) -> String;


pub fn default_formatter(response: &WitcherPacket) -> String {
    format!("{}", response)
}


pub enum ScriptsReloadResponseType {
    Started,
    Log(String),
    Warn {
        file: String,
        line: String,
        msg: String
    },
    Error {
        file: String,
        line: String,
        msg: String
    },
    Finished(bool)
}

pub fn scripts_reload_response_type(response: &WitcherPacket) -> Result<ScriptsReloadResponseType, String> {

    fn file_line_msg(response: &WitcherPacket) -> (String, String, String) {
        let file = response.payload[3].to_string();
        let line = response.payload[2].to_string();
        let msg = response.payload[4].to_string();
        
        (file, line, msg)
    }

    if response.payload.len() > 2 && response.payload[0] == WitcherPacketData::StringUTF8("ScriptCompiler".into()) {
        if response.payload[1] == WitcherPacketData::StringUTF8("started".into()) {
            return Ok(ScriptsReloadResponseType::Started);
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("log".into()) {
            return Ok(ScriptsReloadResponseType::Log( response.payload[2].to_string() ));
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("warn".into()) {
            let (file, line, msg) = file_line_msg(response);
            return Ok(ScriptsReloadResponseType::Warn{ file, line, msg });
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("error".into()) {
            let (file, line, msg) = file_line_msg(response);
            return Ok(ScriptsReloadResponseType::Error{ file, line, msg });
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("finished".into()) {
            return Ok(ScriptsReloadResponseType::Finished( response.payload[2] == WitcherPacketData::Int8(0) ));
        }
    } 
    
    Err("Invalid packet format".to_string())
}


pub fn scripts_reload_formatter(response: &WitcherPacket) -> String {
    if let Ok(response_type) = scripts_reload_response_type(response) {
        match response_type {
            ScriptsReloadResponseType::Started => {
                "Script compilation started...".to_string()
            }
            ScriptsReloadResponseType::Log(s) => {
                s
            }
            ScriptsReloadResponseType::Warn { file, line, msg } => {
                format!("[Warning] {}({}): {}", file, line, msg )
            }
            ScriptsReloadResponseType::Error { file, line, msg } => {
                format!("[Error] {}({}): {}", file, line, msg )
            }
            ScriptsReloadResponseType::Finished(f) => {
                if f {
                    "Script compilation finished successfully.".to_string()
                } else {
                    "Script compilation finished with errors.".to_string()
                }
            } 
        }

    } else {
        default_formatter(response)
    }
}


pub fn scripts_root_path_formatter(response: &WitcherPacket) -> String {
    if response.payload.len() > 2 
    && response.payload[0] == WitcherPacketData::StringUTF8("ScriptCompiler".into()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("RootPathConfirm".into()) {
        return response.payload[2].to_string();
    }

    default_formatter(response)
}


pub fn scripts_execute_formatter(response: &WitcherPacket) -> String {
    if response.payload.len() > 2 {
        return response.payload[2].to_string();
    }

    default_formatter(response)
}


pub fn mod_list_formatter(response: &WitcherPacket) -> String {
    if response.payload.len() >= 3 
    && response.payload[0] == WitcherPacketData::StringUTF8("scripts".into()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("pkgSyncListing".into()) {
        let mut result = String::new();
        
        if let WitcherPacketData::Int32(installed) = response.payload[2] {
            result += &format!("Mods installed: {}\n", installed);
        }

        let mut mods = Vec::new();
        if response.payload.len() > 3 {
            for i in (3 .. response.payload.len()).step_by(2) {
                let mod_name = response.payload[i].to_string();
                mods.push(mod_name);
            }
        }

        mods.sort();
        mods.iter().for_each(|m| result += &format!("{}\n", m));

        return result;
    }

    default_formatter(response)
}


pub fn opcode_formatter(response: &WitcherPacket) -> String {
    if response.payload.len() == 9
    && response.payload[0] == WitcherPacketData::StringUTF8("ScriptDebugger".into()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("OpcodeBreakdownResponse".into()) {
        // I don't know what most of these magical numbers in the response mean
        // so I'm gonna print out only the stuff that looks anywhere useful
        return format!("{}{}", response.payload[6], response.payload[8]);
    }

    default_formatter(response)
}


struct VarlistEntry {
    pub section: String,
    pub variable: String,
    pub value: String
}

pub fn var_list_formatter(response: &WitcherPacket) -> String {
    if response.payload.len() > 4 
    && response.payload[1] == WitcherPacketData::StringUTF8("vars".into()) {
        let mut result = String::new();

        let tab_line = format!("{}+-{}+-{}\n", "-".repeat(40), "-".repeat(45), "-".repeat(40) );
        result += &tab_line;
        result += &format!("{:40}| {:45}| {}\n", "Section", "Variable", "Value");
        result += &tab_line;
        
        let mut entries = Vec::new();
        for i in (4 .. response.payload.len()).step_by(5) {
            entries.push( VarlistEntry{
                section: response.payload[i+1].to_string(),
                variable: response.payload[i].to_string(),
                value: response.payload[i+2].to_string()
            });
        }

        entries.sort_by(|e1, e2| {
            match e1.section.cmp(&e2.section) {
                std::cmp::Ordering::Equal => e1.variable.cmp(&e2.variable),
                other => other
            }
        });

        entries.iter().for_each(|e| result += &format!("{:40}| {:45}| {}\n", e.section, e.variable, e.value) );

        return result;
    }

    default_formatter(response)
}