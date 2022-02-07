#[allow(unused_imports)]
use crate::packet::WitcherPacketData;

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