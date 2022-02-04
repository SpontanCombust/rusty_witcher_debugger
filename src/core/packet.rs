// extern crate encoding;

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
                let data_bytes = data.to_be_bytes();

                bytes.extend(&constants::TYPE_INT16);
                bytes.extend(data_bytes);
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

    pub fn from_bytes( bytes: &[u8] ) -> Result<WitcherPacketData, &str> {
        if bytes.len() <= 2 {
            return Err("Not enough bytes to yield any data!");
        } 

        let data = Err("Invalid data!");



        data
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

    pub fn from_bytes( bytes: &[u8] ) -> Result<WitcherPacket, &str> {
        let packet = Err("Invalid data!");



        packet
    }
}
