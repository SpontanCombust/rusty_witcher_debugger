pub const NAMESP_SCRIPT_DEBUGGER: &str = "ScriptDebugger";
pub const NAMESP_SCRIPT_PROFILER: &str = "ScriptProfiler";
pub const NAMESP_SCRIPT_COMPILER: &str = "ScriptCompiler";
pub const NAMESP_SCRIPTS: &str = "scripts";
pub const NAMESP_REMOTE: &str = "Remote";
pub const NAMESP_UTILITY: &str = "Utility";
pub const NAMESP_CONFIG: &str = "Config";

pub const CMD_BIND: &str = "BIND";


pub const SCRIPT_DEBUGGER_UNFILTERED_LOCALS: &str = "UnfilteredLocals";
pub const SCRIPT_DEBUGGER_SORT_LOCALS: &str = "SortLocals";
pub const SCRIPT_DEBUGGER_OPCODE_REQUEST: &str = "OpcodeBreakdownRequest";

pub const SCRIPT_COMPILER_ROOT_PATH: &str = "RootPath";

pub const SCRIPTS_RELOAD: &str = "reload";
pub const SCRIPTS_MODLIST: &str = "pkgSync";

pub const CONFIG_VAR: i32 = 0xCC00CC00 as u32 as i32; // rust assumes literals to be positive numbers, so without writing this in decimal an explicit conversion is needed
pub const CONFIG_VAR_LIST: &str = "list";
pub const CONFIG_VAR_SET: &str = "set";
