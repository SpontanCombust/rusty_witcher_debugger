use crate::protocol::*;

use super::Message;


pub trait Request: Message {
    type Params: AssemblePayload + DisassemblePayload;
    type Response: AssemblePayload + DisassemblePayload;

    fn assemble_packet(params: Self::Params) -> WitcherPacket {
        let mut asm = WitcherPacketAssembler::new();

        asm = Self::assemble_payload_header(asm);
        asm = params.assemble_payload(asm);

        asm.finish()
    }
}
