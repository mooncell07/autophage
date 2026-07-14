use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Instruction {
    pub address: String,
    pub bytes: String,
    pub mnemonic: String,
    pub operands: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Disassembly {
    pub count: i64,
    pub instructions: Vec<Instruction>,
}
