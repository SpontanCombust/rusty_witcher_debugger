mod common;
pub use common::*;

pub mod requests;
pub mod notifications;


use crate::protocol::*;


/// An abstraction over data sent to and from the game
pub trait Message: Sized {
    /// Some fixed data at the start of payload that can identify a specific kind of message
    fn assemble_payload_header(asm: WitcherPacketAssembler) -> WitcherPacketAssembler;

    fn id() -> MessageId {
        let header = Self::assemble_payload_header(WitcherPacketAssembler::new())
            .finish_as_payload();

        MessageId { header }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId {
    header: Vec<WitcherPacketData>
}