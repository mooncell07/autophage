use crate::models::{Disassembly, FunctionList};
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

    FetchFunctionList,
    ResultFunctionList(Arc<FunctionList>),

    FetchDisassembly,
    ResultDisassembly(Arc<Disassembly>),
}
