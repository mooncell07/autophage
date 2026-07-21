#[derive(Default, Debug)]
pub enum Viewer {
    FunctionListViewer,
    DecompilationViewer,

    #[default]
    DisassemblyViewer,
}

impl Viewer {
    pub fn move_left(current: &Self) -> Self {
        match current {
            Self::DecompilationViewer => Self::DisassemblyViewer,
            Self::DisassemblyViewer => Self::FunctionListViewer,
            Self::FunctionListViewer => Self::FunctionListViewer,
        }
    }

    pub fn move_right(current: &Self) -> Self {
        match current {
            Self::FunctionListViewer => Self::DisassemblyViewer,
            Self::DisassemblyViewer => Self::DecompilationViewer,
            Self::DecompilationViewer => Self::DecompilationViewer,
        }
    }
}
