use crate::models::{Decompilation, Disassembly, FunctionList};
use std::sync::Arc;
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,

    InitializeHome,

    ResultFunctionList(Arc<FunctionList>),
    ResultDisassembly(Arc<Disassembly>),
    ResultDecompilation(Arc<Decompilation>),
}
