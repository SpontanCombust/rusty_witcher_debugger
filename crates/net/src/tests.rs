#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::{commands, encoding::*, packet::WitcherPacket, packet_data::WitcherPacketData};

    #[test]
    fn packet_data_int8_parse_test() {
        let i = WitcherPacketData::new_int8(18i8);
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 3);
    }
    
    #[test]
    fn packet_data_int16_parse_test() {
        let i = WitcherPacketData::new_int16(25564i16);
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 4);
    }
    
    #[test]
    fn packet_data_int32_parse_test() {
        let i = WitcherPacketData::new_int32(912739132i32);
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 6);
    }
    
    #[test]
    fn packet_data_uint32_parse_test() {
        let i = WitcherPacketData::new_uint32(912739132u32);
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 6);
    }
    
    #[test]
    fn packet_data_int64_parse_test() {
        let i = WitcherPacketData::new_int64(-31742921364135i64);
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 10);
    }
    
    #[test]
    fn packet_data_string_utf8_parse_test() {
        let i = WitcherPacketData::new_string_utf8("Gaderypoluki".into());
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 18);
    }
    
    #[test]
    fn packet_data_string_utf16_parse_test() {
        let i = WitcherPacketData::new_string_utf16("Zażółć gęślą jaźń".into());
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 40);
    }
    
    
    
    
    
    #[test]
    fn command_listen_parse_test() {
        let packets = commands::listen_all();
        for p1 in packets {
            let mut bytes = VecDeque::new();
            p1.encode_into(&mut bytes).unwrap();
            let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        
            assert!( p1 == p2 );
        }
    }
    
    #[test]
    fn command_scripts_reload_parse_test() {
        let p1 = commands::scripts_reload();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
    
    #[test]
    fn command_scripts_root_path_parse_test() {
        let p1 = commands::scripts_root_path();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
    
    #[test]
    fn command_scripts_execute_parse_test() {
        let p1 = commands::scripts_execute("additem('Aerondight', 1)".to_owned());
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
    
    #[test]
    fn command_mod_list_parse_test() {
        let p1 = commands::mod_list();
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
    
    #[test]
    fn command_opcode_parse_test() {
        let p1 = commands::opcode("GetPlayerWitcher".to_owned(), None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );
        
        
        let p1 = commands::opcode("onSpawned".to_owned(), Some("CR4Player".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
    
    #[test]
    fn command_var_list_parse_test() {
        let p1 = commands::var_list(None, None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );
        
        
        let p1 = commands::var_list(Some("VarSection".to_owned()), None);
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );
    
    
        let p1 = commands::var_list(None, Some("VarName".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );
    
    
        let p1 = commands::var_list(Some("VarSection".to_owned()), Some("VarName".to_owned()));
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );
    }
    
    #[test]
    fn command_var_set_parse_test() {
        let p1 = commands::var_set("VarSection".to_owned(), "VarName".to_owned(), "false".to_owned());
        let mut bytes = VecDeque::new();
        p1.encode_into(&mut bytes).unwrap();
        let p2 = WitcherPacket::decode_from(&mut bytes).unwrap();
    
        assert!( p1 == p2 );   
    }
}
