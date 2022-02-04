use crate::packet::WitcherPacket;
use crate::constants;

pub fn bind( namespace: &str ) -> WitcherPacket {
    WitcherPacket::new()
        .append_utf8( &constants::CMD_BIND )
        .append_utf8( &namespace )
}