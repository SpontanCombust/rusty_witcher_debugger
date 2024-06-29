mod common;
use anyhow::Context;
pub use common::*;

pub mod requests;
pub mod notifications;


use crate::protocol::*;


/// An abstraction over data sent to and from the game
pub trait Message: Sized {
    type Header: AssemblePayload + DisassemblePayload + Default;
    type Body: AssemblePayload + DisassemblePayload;

    fn assemble_packet(body: Self::Body) -> WitcherPacket {
        let mut asm = WitcherPacketAssembler::new();

        let header = Self::Header::default();
        asm = header.assemble_payload(asm);
        asm = body.assemble_payload(asm);

        asm.finish()
    }

    fn disassemble_packet(packet: WitcherPacket) -> anyhow::Result<Self::Body> {
        let mut dasm = WitcherPacketDisassembler::new(packet);

        Self::Header::disassemble_payload(&mut dasm).context("Invalid or unknown packet header")?;
        let body = Self::Body::disassemble_payload(&mut dasm).context("Invalid or unknown packet body")?;

        Ok(body)
    }
}
