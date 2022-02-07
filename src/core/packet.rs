use std::{str, io::Read};

use crate::constants;

#[derive(PartialEq, Eq)]
pub enum WitcherPacketData {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    UInt32(u32),
    Int64(i64),
    StringUTF8(String),
    StringUTF16(String)
}

impl WitcherPacketData {
    pub fn size(&self) -> usize {
        match self {
            WitcherPacketData::Int8(_) => 3, // 3 = data_type(2) + data(1)
            WitcherPacketData::Int16(_) => 4, // 4 = data_type(2) + data(2) 
            WitcherPacketData::Int32(_) => 6, // 6 = data_type(2) + data(4)
            WitcherPacketData::UInt32(_) => 6, // 6 = data_type(2) + data(4)
            WitcherPacketData::Int64(_) => 10, // 10 = data_type(2) + data(8)
            WitcherPacketData::StringUTF8(s) => 6 + s.len(), // 6 = data_type(2) + string_size_type(2) + string_size(2)
            WitcherPacketData::StringUTF16(s) => 6 + s.len() * 2, // 6 = data_type(2) + string_size_type(2) + string_size(2)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::<u8>::new();

        match self {
            WitcherPacketData::Int8(data) => {
                bytes.extend(&constants::TYPE_INT8);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::Int16(data) => {
                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::Int32(data) => {
                bytes.extend(&constants::TYPE_INT32);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::UInt32(data) => {
                bytes.extend(&constants::TYPE_UINT32);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::Int64(data) => {
                bytes.extend(&constants::TYPE_INT64);
                bytes.extend(data.to_be_bytes());
            }
            WitcherPacketData::StringUTF8(data) => {
                let len_bytes = ( data.len() as i16 ).to_be_bytes();

                bytes.extend(&constants::TYPE_STRING_UTF8);
                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(len_bytes);
                bytes.extend(data.as_bytes());
            }
            WitcherPacketData::StringUTF16(data) => {
                let len_bytes = ( data.chars().count() as i16 ).to_be_bytes();

                let encoded: Vec<u8> = 
                    data.encode_utf16()
                    .map(|c| c.to_be_bytes())
                    .flatten()
                    .collect();

                bytes.extend(&constants::TYPE_STRING_UTF16);
                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(len_bytes);
                bytes.extend(encoded.as_slice());
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
            offset += 2;
            match type_bytes {
                constants::TYPE_INT8 => {
                    if payload.len() - offset < 1 {
                        return Err(err + "Not enough bytes provided to yield Int8");
                    }
                    datas.push( WitcherPacketData::Int8( i8::from_be_bytes( payload[ offset..(offset + 1) ].try_into().unwrap() ) ) );
                    offset += 1;
                }
                constants::TYPE_INT16 => {
                    if payload.len() - offset < 2 {
                        return Err(err + "Not enough bytes provided to yield Int16");
                    }
                    datas.push( WitcherPacketData::Int16( i16::from_be_bytes( payload[ offset..(offset + 2) ].try_into().unwrap() ) ) );
                    offset += 2;
                }
                constants::TYPE_INT32 => {
                    if payload.len() - offset < 4 {
                        return Err(err + "Not enough bytes provided to yield Int32");
                    }
                    datas.push( WitcherPacketData::Int32( i32::from_be_bytes( payload[ offset..(offset + 4) ].try_into().unwrap() ) ) );
                    offset += 4;
                }
                constants::TYPE_UINT32 => {
                    if payload.len() - offset < 4 {
                        return Err(err + "Not enough bytes provided to yield UInt32");
                    }
                    datas.push( WitcherPacketData::UInt32( u32::from_be_bytes( payload[ offset..(offset + 4) ].try_into().unwrap() ) ) );
                    offset += 4;
                }
                constants::TYPE_INT64 => {
                    if payload.len() - offset < 8 {
                        return Err(err + "Not enough bytes provided to yield Int64");
                    }
                    datas.push( WitcherPacketData::Int64( i64::from_be_bytes( payload[ offset..(offset + 8) ].try_into().unwrap() ) ) );
                    offset += 8;
                }
                constants::TYPE_STRING_UTF8 => {
                    if payload.len() - offset < 4 {
                        return Err(err + "Not enough bytes provided to yield StringUTF8");
                    }
    
                    // Received length shouldn't be negative so we can parse it to u16 instead of i16
                    // bytes 1-2 should be the type of string length, which checking can be ignored
                    let str_len: usize = u16::from_be_bytes( payload[ (offset + 2)..(offset + 4) ].try_into().unwrap() ).into();
                    offset += 4;

                    if payload.len() - offset < str_len {
                        return Err(err + "Provided StringUTF8 length outside of payload bounds");
                    }
                    match str::from_utf8( &payload[ offset..(offset + str_len) ] ) {
                        Ok(s) => {
                            datas.push( WitcherPacketData::StringUTF8(s.to_owned()) );
                            offset += str_len;
                        }
                        Err(e) => {
                            return Err( format!("{}UTF8 conversion error: {}", err, e) );
                        }
                    }
                }
                constants::TYPE_STRING_UTF16 => {
                    if payload.len() - offset < 4 {
                        return Err(err + "Not enough bytes provided to yield StringUTF16");
                    }
    
                    // Received length shouldn't be negative so we can parse it to u16 instead of i16
                    // bytes 1-2 should be the type of string length, which checking can be ignored
                    let str_len: usize = u16::from_be_bytes( payload[ (offset + 2)..(offset + 4) ].try_into().unwrap() ).into();
                    offset += 4;

                    if payload.len() - offset < str_len * 2 {
                        return Err(err + "Provided StringUTF16 length outside of payload bounds");
                    }

                    let decoded: Vec<u16> = 
                        payload[ offset..(offset + str_len * 2) ].chunks_exact(2)
                        .map(|hilo| ((hilo[0] as u16) << 8) + hilo[1] as u16 ) // turn two bytes into one short
                        .collect();

                    match String::from_utf16( decoded.as_slice() ) {
                        Ok(s) => {
                            datas.push( WitcherPacketData::StringUTF16(s) );
                            offset += str_len * 2;
                        }
                        Err(e) => {
                            return Err( format!("{}UTF16 conversion error: {}", err, e) );
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
