use crate::protocol::*;

use super::{Message, WitcherNamespace};


pub trait Notification: Message { 
    type Params: AssemblePayload + DisassemblePayload;

    fn assemble_packet(params: Self::Params) -> WitcherPacket {
        let mut asm = WitcherPacketAssembler::new();

        asm = Self::assemble_payload_header(asm);
        asm = params.assemble_payload(asm);

        asm.finish()
    }
}


#[derive(Debug)]
pub enum ListenToNamespace { }

impl Message for ListenToNamespace {
    fn assemble_payload_header(asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8("BIND")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListenToNamespaceParams {
    pub namesp: WitcherNamespace
}

impl AssemblePayload for ListenToNamespaceParams {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        self.namesp.assemble_payload(asm)
    }
}

impl DisassemblePayload for ListenToNamespaceParams {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        Ok(Self {
            namesp: WitcherNamespace::disassemble_payload(dasm)?
        })
    }
}

impl Notification for ListenToNamespace {
    type Params = ListenToNamespaceParams;
}
