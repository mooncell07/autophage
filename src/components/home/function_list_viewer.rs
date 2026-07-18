use std::sync::Arc;

use ratatui::{
    style::Modifier,
    widgets::{Block, List},
};

use crate::models::FunctionList;

pub struct FunctionListViewer {
    function_list: Arc<FunctionList>,
}

impl Default for FunctionListViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionListViewer {
    pub fn new() -> Self {
        return Self {
            function_list: FunctionList::default().into(),
        };
    }
    pub fn update(&mut self, function_list: &Arc<FunctionList>) {
        self.function_list = Arc::clone(function_list);
    }

    pub fn get_widget(&self) -> List<'static> {
        List::new(
            self.function_list
                .functions
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(Block::bordered().title(" Function List "))
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
    }
}
