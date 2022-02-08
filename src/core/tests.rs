use crate::packet::WitcherPacketData;
use crate::{commands, packet::WitcherPacket};

#[test]
fn packet_data_int8_parse_test() {
    let i = WitcherPacketData::Int8(18i8);
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_int16_parse_test() {
    let i = WitcherPacketData::Int16(25564i16);
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_int32_parse_test() {
    let i = WitcherPacketData::Int32(912739132i32);
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_uint32_parse_test() {
    let i = WitcherPacketData::UInt32(912739132u32);
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_int64_parse_test() {
    let i = WitcherPacketData::Int64(-31742921364135i64);
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_string_utf8_parse_test() {
    let i = WitcherPacketData::StringUTF8("Gaderypoluki".to_owned());
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}

#[test]
fn packet_data_string_utf16_parse_test() {
    let i = WitcherPacketData::StringUTF16("Zażółć gęślą jaźń".to_owned());
    let bytes = i.to_bytes();
    let packets = WitcherPacketData::from_bytes(bytes.as_slice());

    assert!(packets.is_ok());
    let packets = packets.unwrap();

    assert_eq!(packets.len(), 1);
    assert!( packets[0] == i );
}





#[test]
fn command_listen_parse_test() {
    let packets = commands::listen_all();
    for p1 in packets {
        let bytes = p1.to_bytes();
        let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );
    
        assert!(p2.is_ok());
        let p2 = p2.unwrap();
    
        assert!( p1 == p2 );
    }
}

#[test]
fn command_scripts_reload_parse_test() {
    let p1 = commands::scripts_reload();
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}

#[test]
fn command_scripts_root_path_parse_test() {
    let p1 = commands::scripts_root_path();
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}

#[test]
fn command_scripts_execute_parse_test() {
    let p1 = commands::scripts_execute("additem('Aerondight', 1)");
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}

#[test]
fn command_mod_list_parse_test() {
    let p1 = commands::mod_list();
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}

#[test]
fn command_opcode_parse_test() {
    let p1 = commands::opcode("GetPlayerWitcher", None);
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );
    
    
    let p1 = commands::opcode("onSpawned", Some("CR4Player"));
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}

#[test]
fn command_var_list_parse_test() {
    let p1 = commands::var_list(None, None);
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );
    
    
    let p1 = commands::var_list(Some("VarSection"), None);
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );


    let p1 = commands::var_list(None, Some("VarName"));
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );


    let p1 = commands::var_list(Some("VarSection"), Some("VarName"));
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );
}

#[test]
fn command_var_set_parse_test() {
    let p1 = commands::var_set("VarSection", "VarName", "false");
    let bytes = p1.to_bytes();
    let p2 = WitcherPacket::from_stream( &mut bytes.as_slice() );

    assert!(p2.is_ok());
    let p2 = p2.unwrap();

    assert!( p1 == p2 );   
}