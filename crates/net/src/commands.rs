use crate::protocol::{WitcherPacket, WitcherPacketBuilder};
use crate::constants;

/// Listen to game messages coming from given namespace
/// * `namespace` - namespace to listen to
pub fn listen(namespace: String) -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::CMD_BIND)
        .string_utf8(namespace)
        .finish()
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
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_SCRIPTS)
        .string_utf8(constants::SCRIPTS_RELOAD)
        .finish()
}

/// Get root directory path of game scripts
pub fn scripts_root_path() -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_SCRIPT_COMPILER)
        .string_utf8(constants::SCRIPT_COMPILER_ROOT_PATH)
        .finish()
}

/// Run exec function from the game
/// * `command` - exec command to execute in the game
#[allow(overflowing_literals)]
pub fn scripts_execute(command: String) -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_REMOTE)
        .int32(0x12345678)
        .int32(0x81160008)
        .string_utf8(command)
        .finish()
}

/// Get the list of installed mods
pub fn mod_list() -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_SCRIPTS)
        .string_utf8(constants::SCRIPTS_MODLIST)
        .finish()
}

/// Get the opcode of a script function
/// * `func_name` - name of the function
/// * `class_name` - name of the class if the function is a member of that class; pass None if it's not a method
pub fn opcode(func_name: String, class_name: Option<String>) -> WitcherPacket {
    let builder = WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_SCRIPT_DEBUGGER)
        .string_utf8(constants::SCRIPT_DEBUGGER_OPCODE_REQUEST)
        .string_utf16(func_name);
    
    if let Some(class) = class_name {
        builder.int8(1)
            .string_utf16(class)
            .finish()
    } else {
        builder.int8(0)
            .finish()
    }
}

/// Search for config variables
/// * `section` - var section to search; if None is passed searches all sections
/// * `name` - token that should be included in vars; if None is passed searches all variables
pub fn var_list(section: Option<String>, name: Option<String>) -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_CONFIG)
        .int32(constants::CONFIG_VAR)
        .string_utf8(constants::CONFIG_VAR_LIST)
        .string_utf8(section.unwrap_or("".to_owned()))
        .string_utf8(name.unwrap_or("".to_owned()))
        .finish()
}

/// Sets a config variable
/// * `section` - variable's section 
/// * `name` - variable's name 
/// * `value` - variable's new value 
pub fn var_set(section: String, name: String, value: String) -> WitcherPacket {
    WitcherPacketBuilder::new()
        .string_utf8(constants::NAMESP_CONFIG)
        .int32(constants::CONFIG_VAR)
        .string_utf8(constants::CONFIG_VAR_SET)
        .string_utf8(section)
        .string_utf8(name)
        .string_utf16(value)
        .finish()
}





#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{commands, protocol::*};
    
    
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
