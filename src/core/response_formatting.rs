use crate::{packet::WitcherPacket, packet_data::WitcherPacketData};

pub type ResponseFormatter = fn(WitcherPacket) -> String;

pub fn default_formatter(response: WitcherPacket) -> String {
    format!("{}", response)
}

pub fn scripts_reload_formatter(response: WitcherPacket) -> String {
    if response.payload.len() > 2 && response.payload[0] == WitcherPacketData::StringUTF8("ScriptCompiler".to_string()) {
        if response.payload[1] == WitcherPacketData::StringUTF8("started".to_string()) {
            return "Script compilation started...".to_string();
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("log".to_string()) {
            return format!("{}", response.payload[2] );
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("warn".to_string()) {
            return format!("[Warning] {}({}): {}", response.payload[3], response.payload[2], response.payload[4] );
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("error".to_string()) {
            return format!("[Error] {}({}): {}", response.payload[3], response.payload[2], response.payload[4] );
        }
        else if response.payload[1] == WitcherPacketData::StringUTF8("finished".to_string()) {
            if response.payload[2] == WitcherPacketData::Int8(0) {
                return "Script compilation finished successfully.".to_string();
            } else {
                return "Script compilation finished with errors.".to_string();
            }
        }
    }

    default_formatter(response)
}

pub fn scripts_root_path_formatter(response: WitcherPacket) -> String {
    if response.payload.len() > 2 
    && response.payload[0] == WitcherPacketData::StringUTF8("ScriptCompiler".to_string()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("RootPathConfirm".to_string()) {
        return format!("{}", response.payload[2] );
    }

    default_formatter(response)
}

pub fn scripts_execute_formatter(response: WitcherPacket) -> String {
    if response.payload.len() > 2 {
        return format!("{}", response.payload[2] );
    }

    default_formatter(response)
}

pub fn mod_list_formatter(response: WitcherPacket) -> String {
    if response.payload.len() >= 3 
    && response.payload[0] == WitcherPacketData::StringUTF8("scripts".to_string()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("pkgSyncListing".to_string()) {
        let mut result = String::new();
        
        if let WitcherPacketData::Int32(installed) = response.payload[2] {
            result += &format!("Mods installed: {}\n", installed);
        }

        if response.payload.len() > 3 {
            for i in (3 .. response.payload.len()).step_by(2) {
                result += &format!("{}\n", response.payload[i]);
            }
        }

        return result;
    }

    default_formatter(response)
}

pub fn opcode_formatter(response: WitcherPacket) -> String {
    if response.payload.len() == 9
    && response.payload[0] == WitcherPacketData::StringUTF8("ScriptDebugger".to_string()) 
    && response.payload[1] == WitcherPacketData::StringUTF8("OpcodeBreakdownResponse".to_string()) {
        // I don't know what most of these magical numbers in the response mean
        // so I'm gonna print out only the stuff that looks anywhere useful
        return format!("{}{}", response.payload[6], response.payload[8]);
    }

    default_formatter(response)
}

pub fn var_list_formatter(response: WitcherPacket) -> String {
    if response.payload.len() > 4 
    && response.payload[1] == WitcherPacketData::StringUTF8("vars".to_string()) {
        let mut result = String::new();

        let tab_line = format!("{}+-{}+-{}\n", "-".repeat(40), "-".repeat(45), "-".repeat(40) );
        result += &tab_line;
        result += &format!("{:40}| {:45}| {}\n", "Section", "Variable", "Value");
        result += &tab_line;
        
        for i in (4 .. response.payload.len()).step_by(5) {
            result += &format!("{:40}| {:45}| {}\n", response.payload[i+1], response.payload[i], response.payload[i+2]);
        }

        return result;
    }

    default_formatter(response)
}