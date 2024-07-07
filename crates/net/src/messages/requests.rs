use std::path::PathBuf;

use anyhow::Context;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::protocol::*;
use super::{Message, WitcherNamespace};


pub trait Request: Message {
    type Response: Response;
}

pub trait Response: Message {}


#[derive(Debug)]
pub struct ScriptsRootPath;

impl Message for ScriptsRootPath {
    type Id = ScriptsRootPathId;
    type Body = ();
}

impl Request for ScriptsRootPath {
    type Response = ScriptsRootPathResponse;
}


#[derive(Debug, Default)]
pub struct ScriptsRootPathId;

impl AssemblePayload for ScriptsRootPathId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::ScriptCompiler.as_ref())
            .string_utf8("RootPath")
    }
}

impl DisassemblePayload for ScriptsRootPathId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::ScriptCompiler.as_ref())?;
        dasm.fixed_string_utf8("RootPath")?;
        Ok(Self)
    }
}



#[derive(Debug)]
pub struct ScriptsRootPathResponse;

impl Message for ScriptsRootPathResponse {
    type Id = ScriptsRootPathResponseId;
    type Body = ScriptsRootPathResult;
}

impl Response for ScriptsRootPathResponse {}


#[derive(Debug, Default)]
pub struct ScriptsRootPathResponseId;

impl AssemblePayload for ScriptsRootPathResponseId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::ScriptCompiler.as_ref())
            .string_utf8("RootPathConfirm")
    }
}

impl DisassemblePayload for ScriptsRootPathResponseId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::ScriptCompiler.as_ref())?;
        dasm.fixed_string_utf8("RootPathConfirm")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScriptsRootPathResult {
    pub abs_path: PathBuf
}

impl AssemblePayload for ScriptsRootPathResult {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf16(self.abs_path.to_string_lossy())
    }
}

impl DisassemblePayload for ScriptsRootPathResult {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let abs_path = PathBuf::from(dasm.string_utf16().context("abs_path")?.0);

        Ok(Self {
            abs_path
        })
    }
}






#[derive(Debug)]
pub struct ExecuteCommand;

impl Message for ExecuteCommand {
    type Id = ExecuteCommandId;
    type Body = ExecuteCommandParams;
}

impl Request for ExecuteCommand {
    type Response = ExecuteCommandResponse;
}


#[derive(Debug, Default)]
pub struct ExecuteCommandId;

impl AssemblePayload for ExecuteCommandId {
    #[allow(overflowing_literals)]
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::Remote.as_ref())
            .int32(0x12345678) // magic number #1
            .int32(0x81160008) // magic number #2
    }
}

impl DisassemblePayload for ExecuteCommandId {
    #[allow(overflowing_literals)]
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::Remote.as_ref())?;
        dasm.fixed_int32(0x12345678)?;
        dasm.fixed_int32(0x81160008)?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteCommandParams {
    pub cmd: String
}

impl AssemblePayload for ExecuteCommandParams {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(self.cmd)
    }
}

impl DisassemblePayload for ExecuteCommandParams {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let cmd = dasm.string_utf8().context("command")?.0;

        Ok(Self {
            cmd
        })
    }
}



#[derive(Debug)]
pub struct ExecuteCommandResponse;

impl Message for ExecuteCommandResponse {
    type Id = ExecuteCommandResponseId;
    type Body = ExecuteCommandResult;
}

impl Response for ExecuteCommandResponse {}


#[derive(Debug, Default)]
pub struct ExecuteCommandResponseId;

impl AssemblePayload for ExecuteCommandResponseId {
    #[allow(overflowing_literals)]
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.int32(0x12345678)
            .int32(0x81160008)
    }
}

impl DisassemblePayload for ExecuteCommandResponseId {
    #[allow(overflowing_literals)]
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_int32(0x12345678)?;
        dasm.fixed_int32(0x81160008)?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExecuteCommandResult {
    Success {
        log_output: Option<Vec<String>>
    },
    Fail
}

impl ExecuteCommandResult {
    const SPAM_OUTPUT: &'static str = "Spam: Command executed without errors";
    const FAIL_OUTPUT: &'static str = "Warn: Failed to process command";
}

impl AssemblePayload for ExecuteCommandResult {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        let text: String;
        match self {
            ExecuteCommandResult::Success { log_output } => {
                if let Some(log_output) = log_output {
                    text = log_output.join("\n");
                } else {
                    text = Self::SPAM_OUTPUT.to_string();
                }
            }
            ExecuteCommandResult::Fail => {
                text = Self::FAIL_OUTPUT.to_string()
            }
        }

        asm.string_utf8(text)
    }
}

impl DisassemblePayload for ExecuteCommandResult {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let text = dasm.string_utf8().context("exec output")?.0;

        match text.as_str() {
            Self::FAIL_OUTPUT => {
                Ok(Self::Fail)
            },
            Self::SPAM_OUTPUT => {
                Ok(Self::Success { 
                    log_output: None 
                })
            },
            _ => {
                let lines = text.split("\n")
                    .map(|s| s.to_string())
                    .collect();

                Ok(Self::Success { 
                    log_output: Some(lines)
                })
            }
        }
    }
}







#[derive(Debug)]
pub struct ScriptPackages;

impl Message for ScriptPackages {
    type Id = ScriptPackagesId;
    type Body = ();
}

impl Request for ScriptPackages {
    type Response = ScriptPackagesResponse;
}


#[derive(Debug, Default)]
pub struct ScriptPackagesId;

impl AssemblePayload for ScriptPackagesId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::Scripts.as_ref())
            .string_utf8("pkgSync")
    }
}

impl DisassemblePayload for ScriptPackagesId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::Scripts.as_ref())?;
        dasm.fixed_string_utf8("pkgSync")?;
        Ok(Self)
    }
}



#[derive(Debug)]
pub struct ScriptPackagesResponse;

impl Message for ScriptPackagesResponse {
    type Id = ScriptPackagesResponseId;
    type Body = ScriptPackagesResult;
}

impl Response for ScriptPackagesResponse {}


#[derive(Debug, Default)]
pub struct ScriptPackagesResponseId;

impl AssemblePayload for ScriptPackagesResponseId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::Scripts.as_ref())
            .string_utf8("pkgSyncListing")
    }
}

impl DisassemblePayload for ScriptPackagesResponseId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::Scripts.as_ref())?;
        dasm.fixed_string_utf8("pkgSyncListing")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScriptPackagesResult {
    pub packages: Vec<ScriptPackageInfo>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScriptPackageInfo {
    pub package_name: String,
    pub abs_scripts_root_path: PathBuf
}

impl AssemblePayload for ScriptPackagesResult {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        let mut asm = asm.int32(self.packages.len() as i32);

        for p in self.packages {
            asm = asm.string_utf16(p.package_name)
                .string_utf16(p.abs_scripts_root_path.to_string_lossy())
        }

        asm
    }
}

impl DisassemblePayload for ScriptPackagesResult {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let package_count = dasm.int32().context("package_count")?;

        let mut packages = Vec::with_capacity(package_count as usize);
        for _ in 0..package_count {
            let package_name = dasm.string_utf16().context("package_name")?.0;
            let abs_package_path = PathBuf::from(dasm.string_utf16().context("abs_package_path")?.0);

            packages.push(ScriptPackageInfo {
                package_name,
                abs_scripts_root_path: abs_package_path
            });
        }

        Ok(Self {
            packages
        })
    }
}





#[derive(Debug)]
pub struct Opcodes;

impl Message for Opcodes {
    type Id = OpcodesId;
    type Body = OpcodesParams;
}

impl Request for Opcodes {
    type Response = OpcodesResponse;
}


#[derive(Debug, Default)]
pub struct OpcodesId;

impl AssemblePayload for OpcodesId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::ScriptDebugger.as_ref())
            .string_utf8("OpcodeBreakdownRequest")
    }
}

impl DisassemblePayload for OpcodesId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::ScriptDebugger.as_ref())?;
        dasm.fixed_string_utf8("OpcodeBreakdownRequest")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpcodesParams {
    pub func_name: String,
    pub class_name: Option<String>,
}

impl AssemblePayload for OpcodesParams {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        let asm = asm.string_utf16(self.func_name);

        if let Some(class_name) = self.class_name {
            asm.int8(1)
                .string_utf16(class_name)
        } else {
            asm.int8(0)
        }
    }
}

impl DisassemblePayload for OpcodesParams {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let func_name = dasm.string_utf16().context("func_name")?.0;

        let has_class_name = dasm.int8().context("has_class_name")?;
        let class_name;
        if has_class_name == 1 {
            class_name = Some(dasm.string_utf16().context("class_name")?.0);
        } else {
            class_name = None;
        }

        Ok(Self {
            func_name,
            class_name
        })
    }
}



#[derive(Debug)]
pub struct OpcodesResponse;

impl Message for OpcodesResponse {
    type Id = OpcodesResponseId;
    type Body = OpcodesResult;
}

impl Response for OpcodesResponse {}


#[derive(Debug, Default)]
pub struct OpcodesResponseId;

impl AssemblePayload for OpcodesResponseId {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::ScriptDebugger.as_ref())
            .string_utf8("OpcodeBreakdownResponse")
    }
}

impl DisassemblePayload for OpcodesResponseId {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::ScriptDebugger.as_ref())?;
        dasm.fixed_string_utf8("OpcodeBreakdownResponse")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpcodesResult {
    pub breakdowns: Vec<OpcodeBreakdown>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpcodeBreakdown {
    pub line: i32,
    pub opcodes: Vec<String>
}

impl AssemblePayload for OpcodesResult {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        let mut asm = asm
            .int32(1) // unknown
            .string_utf16("") // unknown
            .int32(self.breakdowns.len() as i32);

        for b in self.breakdowns {
            asm = asm
                .int32(b.line)
                .string_utf16(b.opcodes.join("\n"));
        }

        asm
    }
}

impl DisassemblePayload for OpcodesResult {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.int32().context("unknown0")?;
        dasm.string_utf16().context("unknown1")?;

        let breakdowns_count = dasm.int32().context("breakdowns_count")?;
        let mut breakdowns = Vec::with_capacity(breakdowns_count as usize);
        for _ in 0..breakdowns_count {
            let line = dasm.int32().context("line")?;
            let opcodes = dasm.string_utf16().context("opcodes")?;

            let opcodes = opcodes.split("\n")
                .map(|s| s.to_string())
                .collect();

            breakdowns.push(OpcodeBreakdown {
                line,
                opcodes
            });
        }

        Ok(Self {
            breakdowns
        })
    }
}





#[derive(Debug)]
pub struct ConfigVars;

impl Message for ConfigVars {
    type Id = ConfigVarsId;
    type Body = ConfigVarsParams;
}

impl Request for ConfigVars {
    type Response = ConfigVarsResponse;
}


#[derive(Debug, Default)]
pub struct ConfigVarsId;

impl AssemblePayload for ConfigVarsId {
    #[allow(overflowing_literals)]
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(WitcherNamespace::Config.as_ref())
            .int32(0xCC00CC00)
            .string_utf8("list")
    }
}

impl DisassemblePayload for ConfigVarsId {
    #[allow(overflowing_literals)]
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_string_utf8(WitcherNamespace::Config.as_ref())?;
        dasm.fixed_int32(0xCC00CC00)?;
        dasm.fixed_string_utf8("list")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfigVarsParams {
    pub section_filter: Option<String>,
    pub name_filter: Option<String>
}

impl AssemblePayload for ConfigVarsParams {
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.string_utf8(self.section_filter.unwrap_or_default())
            .string_utf8(self.name_filter.unwrap_or_default())
    }
}

impl DisassemblePayload for ConfigVarsParams {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let section_filter = dasm.string_utf8().context("section_filter")?.0;
        let name_filter = dasm.string_utf8().context("name_filter")?.0;

        let section_filter = if section_filter.is_empty() { None } else { Some(section_filter) };
        let name_filter = if name_filter.is_empty() { None } else { Some(name_filter) };

        Ok(Self {
            section_filter,
            name_filter
        })
    }
}



#[derive(Debug)]
pub struct ConfigVarsResponse;

impl Message for ConfigVarsResponse {
    type Id = ConfigVarsResponseId;
    type Body = ConfigVarsResult;
}

impl Response for ConfigVarsResponse {}


#[derive(Debug, Default)]
pub struct ConfigVarsResponseId;

impl AssemblePayload for ConfigVarsResponseId {
    #[allow(overflowing_literals)]
    fn assemble_payload(self, asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        asm.int32(0xCC00CC00)
            .string_utf8("vars")
    }
}

impl DisassemblePayload for ConfigVarsResponseId {
    #[allow(overflowing_literals)]
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        dasm.fixed_int32(0xCC00CC00)?;
        dasm.fixed_string_utf8("vars")?;
        Ok(Self)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfigVarsResult {
    pub vars: Vec<ConfigVarInfo>
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfigVarInfo {
    pub section: String,
    pub name: String,
    pub value: String,
    
    /// 1 - bool, 2 - int, 3 - float, 4 - string, 0 - used as EOF
    pub data_type: i8,
    pub _unknown0: i8
}

impl AssemblePayload for ConfigVarsResult {
    fn assemble_payload(self, mut asm: WitcherPacketAssembler) -> WitcherPacketAssembler {
        for var in self.vars {
            asm = asm.int8(var.data_type)
                .int8(var._unknown0)
                .string_utf8(var.name)
                .string_utf8(var.section)
                .string_utf8(var.value);
        }

        asm.int8(0)
    }
}

impl DisassemblePayload for ConfigVarsResult {
    fn disassemble_payload(dasm: &mut WitcherPacketDisassembler) -> anyhow::Result<Self> {
        let mut vars = Vec::new();
        loop {
            let data_type = dasm.int8().context("data_type")?;
            if data_type == 0 {
                break;
            }

            let _unknown0 = dasm.int8().context("unknown0")?;
            let name = dasm.string_utf8().context("name")?.0;
            let section = dasm.string_utf8().context("section")?.0;
            let value = dasm.string_utf8().context("value")?.0;

            vars.push(ConfigVarInfo {
                data_type,
                _unknown0,
                name,
                section,
                value
            })
        }

        Ok(Self {
            vars
        })
    }
}








#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;


    #[test]
    fn scripts_root_path_req_encode_test() {
        let param1 = ();
        let packet1 = ScriptsRootPath::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ScriptsRootPath::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn scripts_root_path_resp_encode_test() {
        let param1 = ScriptsRootPathResult {
            abs_path: r"C:\Program Files\GOG\Witcher 3\content\content0\scripts".into()
        };
        let packet1 = ScriptsRootPathResponse::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ScriptsRootPathResponse::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn exec_command_req_encode_test() {
        let param1 = ExecuteCommandParams {
            cmd: "additem('griffin_sword', 1)".into()
        };
        let packet1 = ExecuteCommand::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ExecuteCommand::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn exec_command_resp_encode_test() {
        {
            let param1 = ExecuteCommandResult::Success { 
                log_output: None 
            };
            let packet1 = ExecuteCommandResponse::assemble_packet(param1.clone());
    
            let mut bytes = VecDeque::new();
            packet1.encode_into(&mut bytes).unwrap();
    
            let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
            let param2 = ExecuteCommandResponse::disassemble_packet(packet2.clone()).unwrap();
    
            assert_eq!(packet1, packet2);
            assert_eq!(param1, param2);
        }
        {
            let param1 = ExecuteCommandResult::Success { 
                log_output: Some(vec![
                    "Hello".into(),
                    "World!".into()
                ]) 
            };
            let packet1 = ExecuteCommandResponse::assemble_packet(param1.clone());
    
            let mut bytes = VecDeque::new();
            packet1.encode_into(&mut bytes).unwrap();
    
            let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
            let param2 = ExecuteCommandResponse::disassemble_packet(packet2.clone()).unwrap();
    
            assert_eq!(packet1, packet2);
            assert_eq!(param1, param2);
        }
        {
            let param1 = ExecuteCommandResult::Fail;
            let packet1 = ExecuteCommandResponse::assemble_packet(param1.clone());
    
            let mut bytes = VecDeque::new();
            packet1.encode_into(&mut bytes).unwrap();
    
            let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
            let param2 = ExecuteCommandResponse::disassemble_packet(packet2.clone()).unwrap();
    
            assert_eq!(packet1, packet2);
            assert_eq!(param1, param2);
        }
    }

    #[test]
    fn script_packages_req_encode_test() {
        let param1 = ();
        let packet1 = ScriptPackages::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ScriptPackages::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn script_packages_resp_encode_test() {
        let param1 = ScriptPackagesResult {
            packages: vec![
                ScriptPackageInfo {
                    package_name: "content0".into(),
                    abs_scripts_root_path: r"C:\Program Files\GOG\Witcher 3\content\content0\scripts".into()
                },
                ScriptPackageInfo {
                    package_name: "modTest1".into(),
                    abs_scripts_root_path: r"C:\Program Files\GOG\Witcher 3\Mods\modTest1\content\scripts".into()
                },
                ScriptPackageInfo {
                    package_name: "modSharedUtils".into(),
                    abs_scripts_root_path: r"C:\Program Files\GOG\Witcher 3\Mods\modSharedUtils\content\scripts".into()
                }
            ]
        };
        let packet1 = ScriptPackagesResponse::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ScriptPackagesResponse::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn opcodes_req_encode_test() {
        let param1 = OpcodesParams {
            class_name: Some("CR4Player".into()),
            func_name: "IsCiri".into()
        };
        let packet1 = Opcodes::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = Opcodes::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn opcodes_resp_encode_test() {
        let param1 = OpcodesResult {
            breakdowns: vec![
                OpcodeBreakdown {
                    line: 123,
                    opcodes: vec!["opcode1".into(), "opcode2".into()]
                },
                OpcodeBreakdown {
                    line: 125,
                    opcodes: vec!["Opcode3".into()]
                }
            ]
        };
        let packet1 = OpcodesResponse::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = OpcodesResponse::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn config_vars_req_encode_test() {
        let param1 = ConfigVarsParams {
            section_filter: Some("Graphics".into()),
            name_filter: None,
        };
        let packet1 = ConfigVars::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ConfigVars::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }

    #[test]
    fn config_vars_resp_encode_test() {
        let param1 = ConfigVarsResult {
            vars: vec![
                ConfigVarInfo {
                    section: "Graphics".into(),
                    name: "FXAA".into(),
                    value: "true".into(),
                    data_type: 1,
                    _unknown0: 0,
                },
                ConfigVarInfo {
                    section: "Graphics".into(),
                    name: "Anisotropic Filtering".into(),
                    value: "8".into(),
                    data_type: 2,
                    _unknown0: 0,
                },
                ConfigVarInfo {
                    section: "Graphics".into(),
                    name: "Shadow Distance".into(),
                    value: "50.25".into(),
                    data_type: 3,
                    _unknown0: 0,
                }
            ]
        };
        let packet1 = ConfigVarsResponse::assemble_packet(param1.clone());

        let mut bytes = VecDeque::new();
        packet1.encode_into(&mut bytes).unwrap();

        let packet2 = WitcherPacket::decode_from(&mut bytes).unwrap();
        let param2 = ConfigVarsResponse::disassemble_packet(packet2.clone()).unwrap();

        assert_eq!(packet1, packet2);
        assert_eq!(param1, param2);
    }
}