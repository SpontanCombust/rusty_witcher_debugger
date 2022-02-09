use std::{io::Read};

use crate::{packet_data::WitcherPacketData, constants};


#[derive(Default, PartialEq, Eq)]
pub struct WitcherPacket {
    payload: Vec<WitcherPacketData>
}

impl WitcherPacket {
    pub fn new() -> WitcherPacket {
        WitcherPacket::default()
    }

    pub fn size(&self) -> usize {
        // accumulate sizes of packet datas
        self.payload.iter().fold(6, |acc, p| acc + p.size() ) // 6 = head(2) + payload_size(2) + tail(2)
    }

    fn append(mut self, data: WitcherPacketData ) -> Self {
        self.payload.push(data);
        self
    }

    pub fn append_int8(self, data: i8) -> Self {
        self.append( WitcherPacketData::Int8(data) )
    }

    pub fn append_int16(self, data: i16) -> Self {
        self.append( WitcherPacketData::Int16(data) )
    }

    pub fn append_int32(self, data: i32) -> Self {
        self.append( WitcherPacketData::Int32(data) )
    }
    
    pub fn append_uint32(self, data: u32) -> Self {
        self.append( WitcherPacketData::UInt32(data) )
    }

    pub fn append_int64(self, data: i64) -> Self {
        self.append( WitcherPacketData::Int64(data) )
    }

    pub fn append_utf8(self, data: &str) -> Self {
        self.append( WitcherPacketData::StringUTF8(data.to_owned()) )
    }

    pub fn append_utf16(self, data: &str) -> Self {
        self.append( WitcherPacketData::StringUTF16(data.to_owned()) )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();
        let packet_length_bytes = (self.size() as i16).to_be_bytes();

        bytes.extend(&constants::PACKET_HEAD);
        bytes.extend(packet_length_bytes);
        for payload_data in &self.payload {
            bytes.extend(payload_data.to_bytes());
        }
        bytes.extend(&constants::PACKET_TAIL);

        bytes
    }

    pub fn from_stream<S>( stream: &mut S ) -> Result<WitcherPacket, String> where S: Read {
        let err = String::from("Failed to read packet from stream - ");
        let mut short_buffer = [0,0];
        let mut payload_buffer = Vec::<u8>::with_capacity(512);
        let mut packet = WitcherPacket::new();

        if let Err(e) = stream.read_exact(&mut short_buffer) {
            return Err(err + &e.to_string());
        }
        if short_buffer != constants::PACKET_HEAD {
            return Err(err + "Incorrect packet head");
        }

        if let Err(e) = stream.read_exact(&mut short_buffer) {
            return Err(err + &e.to_string());
        }
        let packet_len: u16 = u16::from_be_bytes(short_buffer);
        if packet_len > 6 {
            if let Err(e) = stream.take(packet_len as u64 - 6).read_to_end(&mut payload_buffer) {
                return Err(err + &e.to_string());
            }        
        }

        if let Err(e) = stream.read_exact(&mut short_buffer) {
            return Err(err + &e.to_string());
        }
        if short_buffer != constants::PACKET_TAIL {
            return Err(err + "Incorrect packet tail");
        }

        match WitcherPacketData::from_bytes(&payload_buffer) {
            Ok(v) => {
                packet.payload = v;
            }
            Err(e) => {
                return Err(err + &e); 
            }
        }

        Ok(packet)
    }
}



impl std::fmt::Display for WitcherPacket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("---------------------------------------------------\n")?;
        f.write_fmt( format_args!("                PACKET ({} bytes)\n", self.size()) )?;
        f.write_str("---------------------------------------------------")?;
        for p in &self.payload {
            f.write_fmt( format_args!("\n{}\n", p) )?;
        }
        f.write_str("---------------------------------------------------")?;
        
        Ok(())
    }
}
