use std::sync::Arc;

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, List, ListState},
};

use crate::models::FunctionList;

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

    pub fn render(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let l = List::new(
            self.function_list
                .functions
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(Block::default().title(" Functions ").borders(Borders::ALL))
        .highlight_symbol("-> ");

        frame.render_stateful_widget(l, area, &mut self.state);
        Ok(())
    }
}
