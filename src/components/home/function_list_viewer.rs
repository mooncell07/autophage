use std::sync::Arc;

use ratatui::widgets::{Block, Borders, List, ListState};

use crate::models::{Function, FunctionList};

pub struct FunctionListViewer {
    function_list: Arc<FunctionList>,
    pub state: ListState,
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
            state: ListState::default(),
        };
    }

    pub fn update(&mut self, function_list: &Arc<FunctionList>) {
        self.function_list = Arc::clone(function_list);
    }

    pub fn get_function_address(&self, index: usize) -> String {
        self.function_list.functions[index].address.clone()
    }

    pub fn get_widget(&self) -> List<'static> {
        List::new(
            self.function_list
                .functions
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(Block::default().title(" Functions ").borders(Borders::ALL))
        .highlight_symbol("-> ")
    }
}
