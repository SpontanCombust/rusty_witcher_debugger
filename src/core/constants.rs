pub const GAME_PORT: &str = "37001";

pub const PACKET_HEAD: [u8; 2] = [0xDE, 0xAD];
pub const PACKET_TAIL: [u8; 2] = [0xBE, 0xEF];

pub const CMD_BIND: &str = "BIND";

pub const TYPE_INT16: [u8; 2] = [0x81, 0x16];
pub const TYPE_STRING_UTF8: [u8; 2] = [0xAC, 0x08];