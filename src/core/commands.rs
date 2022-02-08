use crate::packet::WitcherPacket;
use crate::constants;

/// Listen to game messages coming from given namespace
/// * `namespace` - namespace to listen to
pub fn listen(namespace: &str) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::CMD_BIND)
        .append_utf8(namespace)
}

/// Listen to game messages coming from all namespaces
pub fn listen_all() -> Vec<WitcherPacket> {
    vec![
        listen(constants::NAMESP_SCRIPT_COMPILER),
        listen(constants::NAMESP_SCRIPT_DEBUGGER),
        listen(constants::NAMESP_SCRIPT_PROFILER),
        listen(constants::NAMESP_SCRIPTS),
        listen(constants::NAMESP_REMOTE),
        listen(constants::NAMESP_UTILITY),
        listen(constants::NAMESP_CONFIG)
    ]
}

/// Reload game scripts
pub fn scripts_reload() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_SCRIPTS)
        .append_utf8(constants::SCRIPTS_RELOAD)
}

/// Get root directory path of game scripts
pub fn scripts_root_path() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_SCRIPT_COMPILER)
        .append_utf8(constants::SCRIPT_COMPILER_ROOT_PATH)
}

/// Run exec function from the game
/// * `command` - exec command to execute in the game
#[allow(overflowing_literals)]
pub fn scripts_execute(command: &str) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_REMOTE)
        .append_int32(0x12345678)
        .append_int32(0x81160008)
        .append_utf8(command)
}

/// Get the list of installed mods
pub fn mod_list() -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_SCRIPTS)
        .append_utf8(constants::SCRIPTS_MODLIST)
}

/// Get the opcode of a script function
/// * `func_name` - name of the function
/// * `class_name` - name of the class if the function is a member of that class; pass None if it's not a method
pub fn opcode(func_name: &str, class_name: Option<&str>) -> WitcherPacket {
    let packet = WitcherPacket::new()
        .append_utf8(constants::NAMESP_SCRIPT_DEBUGGER)
        .append_utf8(constants::SCRIPT_DEBUGGER_OPCODE_REQUEST)
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
pub fn var_list(section: Option<&str>, name: Option<&str>) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_CONFIG)
        .append_int32(constants::CONFIG_VAR)
        .append_utf8(constants::CONFIG_VAR_LIST)
        .append_utf8(section.unwrap_or(""))
        .append_utf8(name.unwrap_or(""))
}

/// Sets a config variable
/// * `section` - variable's section 
/// * `name` - variable's name 
/// * `value` - variable's new value 
pub fn var_set(section: &str, name: &str, value: &str) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8(constants::NAMESP_CONFIG)
        .append_int32(constants::CONFIG_VAR)
        .append_utf8(constants::CONFIG_VAR_SET)
        .append_utf8(section)
        .append_utf8(name)
        .append_utf16(value)
}