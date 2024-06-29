use std::path::PathBuf;

use anyhow::{bail, Context};

use crate::protocol::*;
use super::{Message, WitcherNamespace};


pub trait Notification: Message { }


#[derive(Debug)]
pub struct ListenToNamespace;

impl Message for ListenToNamespace {
    type Id = ListenToNamespaceId;
    type Body = ListenToNamespaceParams;
}

impl Notification for ListenToNamespace {}


#[derive(Debug, Default)]
pub struct ListenToNamespaceId;

impl AssemblePayload for ListenToNamespaceId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8("BIND")
    }
}

impl DisassemblePayload for ListenToNamespaceId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8("BIND")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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





#[derive(Debug)]
pub struct ReloadScripts;

impl Message for ReloadScripts {
    type Id = ReloadScriptsId;
    type Body = ();
}

impl Notification for ReloadScripts {}


#[derive(Debug, Default)]
pub struct ReloadScriptsId;

impl AssemblePayload for ReloadScriptsId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::Scripts.as_ref())
            .string_utf8("reload")
    }
}

impl DisassemblePayload for ReloadScriptsId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::Scripts.as_ref())?;
        dasm.fixed_string_utf8("reload")?;
        Ok(Self)
    }
}





#[derive(Debug)]
pub struct ScriptsReloadProgress;

impl Message for ScriptsReloadProgress {
    type Id = ScriptsReloadProgressId;
    type Body = ScriptsReloadProgressParams;
}

impl Notification for ScriptsReloadProgress {}


#[derive(Debug, Default)]
pub struct ScriptsReloadProgressId;

impl AssemblePayload for ScriptsReloadProgressId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::ScriptCompiler.as_ref())
    }
}

impl DisassemblePayload for ScriptsReloadProgressId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::ScriptCompiler.as_ref())?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScriptsReloadProgressParams {
    Started,
    Log {
        message: String
    },
    Warn {
        line: u32,
        local_script_path: PathBuf,
        message: String
    },
    Error {
        line: u32,
        local_script_path: PathBuf,
        message: String
    },
    Finished {
        success: bool
    }
}

impl AssemblePayload for ScriptsReloadProgressParams {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        match self {
            ScriptsReloadProgressParams::Started => {
                asm.string_utf8("started")
                    .int8(0) // unknown
                    .int8(1) // unknown
            }
            ScriptsReloadProgressParams::Log { message } => {
                asm.string_utf8("log")
                    .string_utf16(message)
            }
            ScriptsReloadProgressParams::Warn { line, local_script_path, message } => {
                asm.string_utf8("warn")
                    .uint32(line)
                    .string_utf16(local_script_path.to_string_lossy())
                    .string_utf16(message)
            }
            ScriptsReloadProgressParams::Error { line, local_script_path, message } => {
                asm.string_utf8("error")
                    .uint32(line)
                    .string_utf16(local_script_path.to_string_lossy())
                    .string_utf16(message)
            }
            ScriptsReloadProgressParams::Finished { success } => {
                asm.string_utf8("finished")
                    .int8(if success {0} else {1})
            }
        }
    }
}

impl DisassemblePayload for ScriptsReloadProgressParams {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let kind = dasm.string_utf8()?;

        match kind.as_str() {
            "started" => {
                dasm.int8().context("started::unknown0")?;
                dasm.int8().context("started::unknown1")?; // unknown

                Ok(Self::Started)
            },
            "log" => {
                let message = dasm.string_utf16().context("log::message")?.0;

                Ok(Self::Log { message })
            },
            "warn" => {
                let line = dasm.uint32().context("warn::line")?;
                let local_script_path = PathBuf::from(dasm.string_utf16().context("warn::local_script_path")?.0);
                let message = dasm.string_utf16().context("warn::message")?.0;

                Ok(Self::Warn { line, local_script_path, message })
            },
            "error" => {
                let line = dasm.uint32().context("error::line")?;
                let local_script_path = PathBuf::from(dasm.string_utf16().context("error::local_script_path")?.0);
                let message = dasm.string_utf16().context("error::message")?.0;

                Ok(Self::Error { line, local_script_path, message })
            },
            "finished" => {
                let success = dasm.int8().context("finished::return_code")? == 0;

                Ok(Self::Finished { success })
            },
            _ => bail!("Unknown script reload progress kind")
        }
    }
}