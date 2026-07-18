use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Instruction {
    pub address: String,
    pub bytes: String,
    pub mnemonic: String,
    pub operands: Vec<String>,
}

#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
pub struct Disassembly {
    pub count: i64,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Function {
    pub address: String,
    pub calling_convention: String,
    pub comment: Option<String>,
    pub entry_point: String,
    pub name: String,
    pub signature: String,
    pub size: i64,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct FunctionList {
    pub count: i64,
    pub functions: Vec<Function>,
}
