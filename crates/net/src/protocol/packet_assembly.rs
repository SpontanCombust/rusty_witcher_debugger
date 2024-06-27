use super::{StringUtf16, StringUtf8, WitcherPacket, WitcherPacketData};


#[derive(Debug, Clone)]
pub struct WitcherPacketAssembler {
    payload: Vec<WitcherPacketData>
}

impl WitcherPacketAssembler {
    #[inline]
    pub fn new() -> Self {
        Self {
            payload: Vec::new()
        }
    }

    #[inline]
    pub fn int8(self, data: i8) -> Self {
        self.append(WitcherPacketData::new_int8(data))
    }

    #[inline]
    pub fn int16(self, data: i16) -> Self {
        self.append(WitcherPacketData::new_int16(data))
    }

    #[inline]
    pub fn int32(self, data: i32) -> Self {
        self.append(WitcherPacketData::new_int32(data))
    }
    
    #[inline]
    pub fn uint32(self, data: u32) -> Self {
        self.append(WitcherPacketData::new_uint32(data))
    }

    #[inline]
    pub fn int64(self, data: i64) -> Self {
        self.append(WitcherPacketData::new_int64(data))
    }

    #[inline]
    pub fn string_utf8<S: Into<StringUtf8>>(self, data: S) -> Self {
        self.append(WitcherPacketData::new_string_utf8(data.into()))
    }

    #[inline]
    pub fn string_utf16<S: Into<StringUtf16>>(self, data: S) -> Self {
        self.append(WitcherPacketData::new_string_utf16(data.into()))
    }

    #[inline]
    pub fn finish(self) -> WitcherPacket {
        WitcherPacket {
            payload: self.payload
        }
    }


    #[inline]
    fn append(mut self, data: WitcherPacketData) -> Self {
        self.payload.push(data);
        self
    }
}