use anyhow::Context;
use strum_macros::EnumTryAs;

use super::encoding::*;


#[derive(Clone, PartialEq, Eq, Hash, EnumTryAs)]
pub enum WitcherPacketData {
    Int8(Tagged<i8>),
    Int16(Tagged<i16>),
    Int32(Tagged<i32>),
    UInt32(Tagged<u32>),
    Int64(Tagged<i64>),
    StringUTF8(Tagged<StringUtf8>),
    StringUTF16(Tagged<StringUtf16>),
    /// for cases where the data tag was not recognized
    Unknown(UnknownTag)
}

impl WitcherPacketData {
    pub fn new_int8(n: i8) -> Self {
        Self::Int8(Tagged::new(n))
    }

    pub fn new_int16(n: i16) -> Self {
        Self::Int16(Tagged::new(n))
    }

    pub fn new_int32(n: i32) -> Self {
        Self::Int32(Tagged::new(n))
    }

    pub fn new_uint32(n: u32) -> Self {
        Self::UInt32(Tagged::new(n))
    }

    pub fn new_int64(n: i64) -> Self {
        Self::Int64(Tagged::new(n))
    }

    pub fn new_string_utf8<S: Into<StringUtf8>>(s: S) -> Self {
        Self::StringUTF8(Tagged::new(s.into()))
    }

    pub fn new_string_utf16<S: Into<StringUtf16>>(s: S) -> Self {
        Self::StringUTF16(Tagged::new(s.into()))
    }

    pub fn new_unknown(t: UnknownTag) -> Self {
        Self::Unknown(t)
    }
}

impl Encode for WitcherPacketData {
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        match self {
            WitcherPacketData::Int8(d) => d.encode_into(stream),
            WitcherPacketData::Int16(d) => d.encode_into(stream),
            WitcherPacketData::Int32(d) => d.encode_into(stream),
            WitcherPacketData::UInt32(d) => d.encode_into(stream),
            WitcherPacketData::Int64(d) => d.encode_into(stream),
            WitcherPacketData::StringUTF8(d) => d.encode_into(stream),
            WitcherPacketData::StringUTF16(d) => d.encode_into(stream),
            WitcherPacketData::Unknown(d) => d.encode_into(stream),
        }
    }
}

impl DynSizedEncode for WitcherPacketData {
    fn encoded_size(&self) -> usize {
        match self {
            WitcherPacketData::Int8(d) => d.encoded_size(),
            WitcherPacketData::Int16(d) => d.encoded_size(),
            WitcherPacketData::Int32(d) => d.encoded_size(),
            WitcherPacketData::UInt32(d) => d.encoded_size(),
            WitcherPacketData::Int64(d) => d.encoded_size(),
            WitcherPacketData::StringUTF8(d) => d.encoded_size(),
            WitcherPacketData::StringUTF16(d) => d.encoded_size(),
            WitcherPacketData::Unknown(d) => d.encoded_size(),
        }
    }
}

impl Decode for WitcherPacketData {
    fn decode_from<S: std::io::Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut tag = [0u8; 2];
        stream.read_exact(&mut tag).context("Failed to read a tag")?;

        match tag {
            i8::ENCODING_TAG => {
                let d = i8::decode_from(stream)?;
                Ok(WitcherPacketData::new_int8(d))
            },
            i16::ENCODING_TAG => {
                let d = i16::decode_from(stream)?;
                Ok(WitcherPacketData::new_int16(d))
            },
            i32::ENCODING_TAG => {
                let d = i32::decode_from(stream)?;
                Ok(WitcherPacketData::new_int32(d))
            },
            u32::ENCODING_TAG => {
                let d = u32::decode_from(stream)?;
                Ok(WitcherPacketData::new_uint32(d))
            },
            i64::ENCODING_TAG => {
                let d = i64::decode_from(stream)?;
                Ok(WitcherPacketData::new_int64(d))
            },
            StringUtf8::ENCODING_TAG => {
                let d = StringUtf8::decode_from(stream)?;
                Ok(WitcherPacketData::new_string_utf8(d))
            },
            StringUtf16::ENCODING_TAG => {
                let d = StringUtf16::decode_from(stream)?;
                Ok(WitcherPacketData::new_string_utf16(d))
            },
            _ => {
                Ok(WitcherPacketData::new_unknown(tag.into()))
            }
        }
    }
}

impl std::fmt::Debug for WitcherPacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int8(i) => {
                write!(f, "Type: Int8\nValue: {}", i)
            }
            Self::Int16(i) => {
                write!(f, "Type: Int16\nValue: {}", i)
            }
            Self::Int32(i) => {
                write!(f, "Type: Int32\nValue: {}", i)
            }
            Self::UInt32(i) => {
                write!(f, "Type: UInt32\nValue: {}", i)
            }
            Self::Int64(i) => {
                write!(f, "Type: Int64\nValue: {}", i)
            }
            Self::StringUTF8(s) => {
                write!(f, "Type: StringUTF8\nLength: {}\nValue: {}", s.chars().count(), s)
            }
            Self::StringUTF16(s) => {
                write!(f, "Type: StringUTF16\nLength: {}\nValue: {}", s.chars().count(), s)
            },
            Self::Unknown(b) => {
                write!(f, "Type: Unknown\nValue: {:?}", b)
            }
        }
    }
}

impl std::fmt::Display for WitcherPacketData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Int8(i) => format!("{}", i),
            Self::Int16(i) => format!("{}", i),
            Self::Int32(i) => format!("{}", i),
            Self::UInt32(i) => format!("{}", i),
            Self::Int64(i) => format!("{}", i),
            Self::StringUTF8(s) => format!("{}", s),
            Self::StringUTF16(s) => format!("{}", s),
            Self::Unknown(s) => format!("{:?}", s)
        };

        if let Some(width) = f.width() {
            write!(f, "{:w$}", s, w = width)
        } else {
            write!(f, "{}", s)
        }
    }
}





#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::protocol::{encoding::*, packet_data::WitcherPacketData};

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
        let i = WitcherPacketData::new_string_utf8("Gaderypoluki");
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 18);
    }
    
    #[test]
    fn packet_data_string_utf16_parse_test() {
        let i = WitcherPacketData::new_string_utf16("Zażółć gęślą jaźń");
        let mut bytes = VecDeque::new();
        i.encode_into(&mut bytes).unwrap();
        let packet = WitcherPacketData::decode_from(&mut bytes).unwrap();
        let bytes_read = packet.encoded_size();
    
        assert_eq!(packet, i);
        assert_eq!(bytes_read, 40);
    }
    
}