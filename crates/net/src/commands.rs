use crate::packet::WitcherPacket;
use crate::constants;

/// Listen to game messages coming from given namespace
/// * `namespace` - namespace to listen to
pub fn listen(namespace: String) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::CMD_BIND)
        .append_utf8(namespace)
}

/// Listen to game messages coming from all namespaces
pub fn listen_all() -> Vec<WitcherPacket> {
    vec![
        listen(constants::NAMESP_SCRIPT_COMPILER.to_owned()),
        listen(constants::NAMESP_SCRIPT_DEBUGGER.to_owned()),
        listen(constants::NAMESP_SCRIPT_PROFILER.to_owned()),
        listen(constants::NAMESP_SCRIPTS.to_owned()),
        listen(constants::NAMESP_REMOTE.to_owned()),
        listen(constants::NAMESP_UTILITY.to_owned()),
        listen(constants::NAMESP_CONFIG.to_owned())
    ]
}

/// Reload game scripts
pub fn scripts_reload() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_SCRIPTS)
        .append_utf8_raw(constants::SCRIPTS_RELOAD)
}

/// Get root directory path of game scripts
pub fn scripts_root_path() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_SCRIPT_COMPILER)
        .append_utf8_raw(constants::SCRIPT_COMPILER_ROOT_PATH)
}

/// Run exec function from the game
/// * `command` - exec command to execute in the game
#[allow(overflowing_literals)]
pub fn scripts_execute(command: String) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_REMOTE)
        .append_int32(0x12345678)
        .append_int32(0x81160008)
        .append_utf8(command)
}

/// Get the list of installed mods
pub fn mod_list() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_SCRIPTS)
        .append_utf8_raw(constants::SCRIPTS_MODLIST)
}

/// Get the opcode of a script function
/// * `func_name` - name of the function
/// * `class_name` - name of the class if the function is a member of that class; pass None if it's not a method
pub fn opcode(func_name: String, class_name: Option<String>) -> WitcherPacket {
    let packet = WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_SCRIPT_DEBUGGER)
        .append_utf8_raw(constants::SCRIPT_DEBUGGER_OPCODE_REQUEST)
        .append_utf16(func_name);
    
    if let Some(class) = class_name {
        return packet.append_int8(1).append_utf16(class);
    } else {
        return packet.append_int8(0);
    }
}

/// Search for config variables
/// * `section` - var section to search; if None is passed searches all sections
/// * `name` - token that should be included in vars; if None is passed searches all variables
pub fn var_list(section: Option<String>, name: Option<String>) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_CONFIG)
        .append_int32(constants::CONFIG_VAR)
        .append_utf8_raw(constants::CONFIG_VAR_LIST)
        .append_utf8(section.unwrap_or("".to_owned()))
        .append_utf8(name.unwrap_or("".to_owned()))
}

/// Sets a config variable
/// * `section` - variable's section 
/// * `name` - variable's name 
/// * `value` - variable's new value 
pub fn var_set(section: String, name: String, value: String) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8_raw(constants::NAMESP_CONFIG)
        .append_int32(constants::CONFIG_VAR)
        .append_utf8_raw(constants::CONFIG_VAR_SET)
        .append_utf8(section)
        .append_utf8(name)
        .append_utf16(value)
}





#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{commands, encoding::*, packet::WitcherPacket};
    
    
    #[test]
    fn command_listen_parse_test() {
        let packets = commands::listen_all();
        for p1 in packets {
            let mut bytes = VecDeque::new();
            p1.encode_into(&mut bytes).unwrap();
            let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        
            assert_eq!(p1, p2);
        }
    }
    
    #[test]
    fn command_scripts_reload_parse_test() {
        let p1 = commands::scripts_reload();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_scripts_root_path_parse_test() {
        let p1 = commands::scripts_root_path();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_scripts_execute_parse_test() {
        let p1 = commands::scripts_execute("additem('Aerondight', 1)".to_owned());
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_mod_list_parse_test() {
        let p1 = commands::mod_list();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_opcode_parse_test() {
        let p1 = commands::opcode("GetPlayerWitcher".to_owned(), None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
        
        
        let p1 = commands::opcode("onSpawned".to_owned(), Some("CR4Player".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_var_list_parse_test() {
        let p1 = commands::var_list(None, None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
        
        
        let p1 = commands::var_list(Some("VarSection".to_owned()), None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    
    
        let p1 = commands::var_list(None, Some("VarName".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    
    
        let p1 = commands::var_list(Some("VarSection".to_owned()), Some("VarName".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
    
    #[test]
    fn command_var_set_parse_test() {
        let p1 = commands::var_set("VarSection".to_owned(), "VarName".to_owned(), "false".to_owned());
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert_eq!(p1, p2); 
    }
}
