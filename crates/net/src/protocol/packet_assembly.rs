use anyhow::Context;

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
        self.push(WitcherPacketData::new_int8(data))
    }

    #[inline]
    pub fn int16(self, data: i16) -> Self {
        self.push(WitcherPacketData::new_int16(data))
    }

    #[inline]
    pub fn int32(self, data: i32) -> Self {
        self.push(WitcherPacketData::new_int32(data))
    }
    
    #[inline]
    pub fn uint32(self, data: u32) -> Self {
        self.push(WitcherPacketData::new_uint32(data))
    }

    #[inline]
    pub fn int64(self, data: i64) -> Self {
        self.push(WitcherPacketData::new_int64(data))
    }

    #[inline]
    pub fn string_utf8<S: Into<StringUtf8>>(self, data: S) -> Self {
        self.push(WitcherPacketData::new_string_utf8(data.into()))
    }

    #[inline]
    pub fn string_utf16<S: Into<StringUtf16>>(self, data: S) -> Self {
        self.push(WitcherPacketData::new_string_utf16(data.into()))
    }

    #[inline]
    pub fn finish(self) -> WitcherPacket {
        WitcherPacket {
            payload: self.payload
        }
    }

    #[inline]
    pub fn finish_as_payload(self) -> Vec<WitcherPacketData> {
        self.payload
    }


    #[inline]
    fn push(mut self, data: WitcherPacketData) -> Self {
        self.payload.push(data);
        self
    }
}



#[derive(Debug, Clone)]
pub struct WitcherPacketDisassembler {
    payload_rev: Vec<WitcherPacketData>
}

impl WitcherPacketDisassembler {
    #[inline]
    pub fn new(mut payload: Vec<WitcherPacketData>) -> Self {
        // reverse now so we can just do pop() later
        payload.reverse();

        Self {
            payload_rev: payload
        }
    }

    #[inline]
    pub fn int8(&mut self) -> anyhow::Result<i8> {
        self.pop()?.try_as_int_8().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn int16(&mut self) -> anyhow::Result<i16> {
        self.pop()?.try_as_int_16().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn int32(&mut self) -> anyhow::Result<i32> {
        self.pop()?.try_as_int_32().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn uint32(&mut self) -> anyhow::Result<u32> {
        self.pop()?.try_as_uint_32().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn int64(&mut self) -> anyhow::Result<i64> {
        self.pop()?.try_as_int_64().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn string_utf8(&mut self) -> anyhow::Result<StringUtf8> {
        self.pop()?.try_as_string_utf_8().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn string_utf16(&mut self) -> anyhow::Result<StringUtf16> {
        self.pop()?.try_as_string_utf_16().map(|t| t.into_inner()).context("Type mismatch")
    }

    #[inline]
    pub fn any(&mut self) -> anyhow::Result<WitcherPacketData> {
        self.pop()
    }


    #[inline]
    fn pop(&mut self) -> anyhow::Result<WitcherPacketData> {
        self.payload_rev.pop().context("No more data found")
    }
}



pub trait AssemblePayload: Sized {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler;
}

pub trait DisassemblePayload: Sized {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self>;
}


impl AssemblePayload for () {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm
    }
}

impl DisassemblePayload for () {
    fn disassemble_payload(_: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        Ok(())
    }
}