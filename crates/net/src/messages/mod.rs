use std::collections::HashSet;

use anyhow::Context;
use crate::protocol::*;


mod common;
pub use common::*;

pub mod requests;
pub use requests::{Request, Response};

pub mod notifications;
pub use notifications::Notification;


/// An abstraction over data sent to and from the game
pub trait Message: Sized {
    // some fixed data format at the beginning of the payload
    type Id: AssemblePayload + DisassemblePayload + Default;
    // variable content of the payload
    type Body: AssemblePayload + DisassemblePayload;


    fn assemble_id() -> MessageId {
        let data = Self::Id::default()
            .assemble_payload(WitcherPacketAssembler::new())
            .finish_as_payload();

        MessageId(data)
    }

    fn assemble_packet(body: Self::Body) -> WitcherPacket {
        let mut asm = WitcherPacketAssembler::new();

        let id = Self::Id::default();
        asm = id.assemble_payload(asm);
        asm = body.assemble_payload(asm);

        asm.finish()
    }

    fn disassemble_packet(packet: WitcherPacket) -> anyhow::Result<Self::Body> {
        let mut dasm = WitcherPacketDisassembler::new(packet);

        Self::Id::disassemble_payload(&mut dasm).context("Invalid or unknown packet id")?;
        let body = Self::Body::disassemble_payload(&mut dasm).context("Invalid or unknown packet body")?;

        Ok(body)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(Vec<WitcherPacketData>);

impl std::borrow::Borrow<[WitcherPacketData]> for MessageId {
    fn borrow(&self) -> &[WitcherPacketData] {
        self.0.as_slice()
    }
}


/// Messages sent to and by the game do not have a fixed format that can identify them.
/// That's why you need to know all message formats in advance and compare beginnings of their payloads to properly identify raw packets.
/// One registry should be used only for messages passed in one direction, i.e. server to client or client to server.
#[derive(Debug)]
pub struct MessageIdRegistry {
    ids: HashSet<MessageId>,
    max_known_id_length: usize
}

impl MessageIdRegistry {
    pub fn new() -> Self {
        Self {
            ids: HashSet::new(),
            max_known_id_length: 0
        }
    }

    pub fn register_message<T>(&mut self) -> MessageId
    where T: Message {
        let id = T::assemble_id();
        let id_length = id.0.len();

        self.ids.insert(id.clone());
        self.max_known_id_length = std::cmp::max(self.max_known_id_length, id_length);
        id
    }

    pub fn probe_message_id(&self, packet: &WitcherPacket) -> Option<MessageId> {
        let mut longest_detected = 0;
        let mut i = 1;
        while i <= packet.payload.len() && i <= self.max_known_id_length {
            let payload_slice = &packet.payload[0..i];
            if self.ids.contains(payload_slice) {
                longest_detected = i;
            }
            i += 1;
        }

        if longest_detected > 0 {
            let payload_slice = &packet.payload[0..longest_detected];
            let id = MessageId(payload_slice.to_owned());
            Some(id)
        } else {
            None
        }
    }


    pub fn register_server_messages(&mut self) {
        self.register_message::<notifications::ScriptsReloadProgress>();

        self.register_message::<requests::ScriptsRootPathResponse>();
        self.register_message::<requests::ExecuteCommandResponse>();
        self.register_message::<requests::ScriptPackagesResponse>();
        self.register_message::<requests::OpcodesResponse>();
        self.register_message::<requests::ConfigVarsResponse>();
    }
}