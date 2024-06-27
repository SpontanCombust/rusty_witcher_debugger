use std::io::Read;

use anyhow::{bail, Context};

use super::encoding::*;
use super::packet_data::WitcherPacketData;


#[derive(Default, Clone, PartialEq, Eq)]
pub struct WitcherPacket {
    pub(crate) payload: Vec<WitcherPacketData>
}

impl WitcherPacket {
    pub const HEAD: [u8; 2] = [0xDE, 0xAD];
    pub const TAIL: [u8; 2] = [0xBE, 0xEF];


    pub fn new() -> WitcherPacket {
        WitcherPacket::default()
    }

    fn append(mut self, data: WitcherPacketData) -> Self {
        self.payload.push(data);
        self
    }

    pub fn append_int8(self, data: i8) -> Self {
        self.append( WitcherPacketData::new_int8(data) )
    }

    pub fn append_int16(self, data: i16) -> Self {
        self.append( WitcherPacketData::new_int16(data) )
    }

    pub fn append_int32(self, data: i32) -> Self {
        self.append( WitcherPacketData::new_int32(data) )
    }
    
    pub fn append_uint32(self, data: u32) -> Self {
        self.append( WitcherPacketData::new_uint32(data) )
    }

    pub fn append_int64(self, data: i64) -> Self {
        self.append( WitcherPacketData::new_int64(data) )
    }

    pub fn append_utf8_raw(self, data: &str) -> Self {
        self.append( WitcherPacketData::new_string_utf8(data.into()) )
    }

    pub fn append_utf8(self, data: String) -> Self {
        self.append( WitcherPacketData::new_string_utf8(data.into()) )
    }

    pub fn append_utf16_raw(self, data: &str) -> Self {
        self.append( WitcherPacketData::new_string_utf16(data.into()) )
    }

    pub fn append_utf16(self, data: String) -> Self {
        self.append( WitcherPacketData::new_string_utf16(data.into()) )
    }


    pub const fn min_encoded_size() -> usize {
        Self::HEAD.len() + i16::ENCODED_SIZE + Self::TAIL.len()
    }
}

impl Encode for WitcherPacket {
    fn encode_into<S: std::io::Write>(&self, stream: &mut S) -> anyhow::Result<()> {
        stream.write_all(&Self::HEAD)?;
        (self.encoded_size() as u16).encode_into(stream)?;
        for data in self.payload.iter() {
            data.encode_into(stream)?;
        }
        stream.write_all(&Self::TAIL)?;
        Ok(())
    }
}

impl DynSizedEncode for WitcherPacket {
    fn encoded_size(&self) -> usize {
        self.payload.iter()
            .fold(Self::min_encoded_size(), |acc, p| acc + p.encoded_size())
    }
}

impl Decode for WitcherPacket {
    fn decode_from<S: Read>(stream: &mut S) -> anyhow::Result<Self> {
        let mut short_buf = [0u8; 2];

        stream.read_exact(&mut short_buf).context("No packet head")?;
        if short_buf != Self::HEAD {
            bail!("Invalid packet head: {:?}", short_buf)
        }

        let mut encoded_size = u16::decode_from(stream).context("Failed to decode packet size")?;
        let mut packet = WitcherPacket::new();

        if encoded_size > Self::min_encoded_size() as u16 {
            encoded_size -= Self::min_encoded_size() as u16;

            while encoded_size > 0 {
                let data = WitcherPacketData::decode_from(stream).context("Failed to decode payload data")?;
                encoded_size = encoded_size.checked_sub(data.encoded_size() as u16).unwrap_or(0);
                packet.payload.push(data);
            }
        }

        stream.read_exact(&mut short_buf).context("No packet tail")?;
        if short_buf != Self::TAIL {
            bail!("Invalid packet tail: {:?}", short_buf)
        }

        Ok(packet)
    }
}



impl std::fmt::Debug for WitcherPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("---------------------------------------------------\n")?;
        f.write_fmt( format_args!("                PACKET ({} bytes)\n", self.encoded_size()) )?;
        f.write_str("---------------------------------------------------")?;
        for p in &self.payload {
            f.write_fmt( format_args!("\n{:?}\n", p) )?;
        }
        f.write_str("---------------------------------------------------")?;
        
        Ok(())
    }
}

impl std::fmt::Display for WitcherPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for p in &self.payload {
            f.write_fmt( format_args!("{} ", p) )?;
        }
        Ok(())
    }
}