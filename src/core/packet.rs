use std::{str, io::Read};

use crate::constants;

#[derive(PartialEq, Eq)]
enum WitcherPacketData {
    Int16(i16),
    StringUTF8(String)
}

impl WitcherPacketData {
    pub fn size(&self) -> usize {
        match self {
            WitcherPacketData::Int16(_) => 4, // 2 = data_type(2) + data(2) 
            WitcherPacketData::StringUTF8(s) => {
                6 + s.len() // 6 = data_type(2) + string_size_type(2) + string_size(2)
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        match self {
            WitcherPacketData::Int16(data) => {
                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::StringUTF8(data) => {
                let len_bytes = ( data.len() as i16 ).to_be_bytes();
                bytes.extend(&constants::TYPE_STRING_UTF8);
                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(len_bytes);
                bytes.extend(data.as_bytes());
            }
        }

        bytes
    }

    pub fn from_bytes( payload: &[u8] ) -> Result<Vec<WitcherPacketData>, String> {
        // size of the payload is always kept on 2 bytes in the packet
        let err = String::from("Failed to parse payload - ");
        let mut offset: usize = 0;
        let mut datas = Vec::new();

        if payload.len() <= 2 {
            return Err(err + "Payload size too small");
        }

        while offset < payload.len() {
            let type_bytes: [u8;2] = payload[ offset..(offset + 2) ].try_into().unwrap();
            match type_bytes {
                constants::TYPE_INT16 => {
                    if payload.len() - offset - 2 < 2 {
                        return Err(err + "Not enough bytes provided to yield Int16");
                    }
                    datas.push( WitcherPacketData::Int16( i16::from_be_bytes( payload[ (offset + 2)..(offset + 4) ].try_into().unwrap() ) ) );
                    offset += 4;
                }
                constants::TYPE_STRING_UTF8 => {
                    if payload.len() - offset - 2 < 4 {
                        return Err(err + "Not enough bytes provided to yield StringUTF8");
                    }
    
                    // Received length shouldn't be negative so we can parse it to u16 instead of i16
                    // bytes 2-4 should be the type of string length, which checking can be ignored
                    let str_len: usize = u16::from_be_bytes( payload[ (offset + 4)..(offset + 6) ].try_into().unwrap() ).into();
                    if payload.len() - offset - 6 < str_len {
                        return Err(err + "Provided StringUTF8 length outside of payload bounds");
                    }
    
                    match str::from_utf8( &payload[ (offset + 6)..(offset + 6 + str_len) ] ) {
                        Ok(s) => {
                            datas.push( WitcherPacketData::StringUTF8(s.to_owned()) );
                            offset += 6 + str_len;
                        }
                        Err(e) => {
                            return Err( format!("{}UTF8 conversion error: {}", err, e) );
                        }
                    }
                }
                _ => return Err( format!("{}Unknown type bytes: {:?}", err, type_bytes) ),
            }
        }

        Ok(datas)
    }
}




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

    pub fn append_int16(self, data: i16) -> Self {
        self.append( WitcherPacketData::Int16(data) )
    }

    pub fn append_utf8(self, data: &str) -> Self {
        self.append( WitcherPacketData::StringUTF8(data.to_owned()) )
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
