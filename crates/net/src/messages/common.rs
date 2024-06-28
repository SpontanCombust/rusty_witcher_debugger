use strum_macros::{AsRefStr, EnumString};

use crate::protocol::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, EnumString)]
pub enum WitcherNamespace {
    #[strum(serialize = "ScriptDebugger")]
    ScriptDebugger,
    #[strum(serialize = "ScriptProfiler")]
    ScriptProfiler,
    #[strum(serialize = "ScriptCompiler")]
    ScriptCompiler,
    #[strum(serialize = "scripts")]
    Scripts,
    #[strum(serialize = "Remote")]
    Remote,
    #[strum(serialize = "Utility")]
    Utility,
    #[strum(serialize = "Config")]
    Config
}

impl AssemblePayload for WitcherNamespace {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(self.as_ref())
    }
}

impl DisassemblePayload for WitcherNamespace {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let s = dasm.string_utf8()?;
        let ns = WitcherNamespace::try_from(s.as_str())?;
        Ok(ns)
    }
}
