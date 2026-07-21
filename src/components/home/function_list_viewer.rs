use std::sync::Arc;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, List, ListState},
};

use crate::{components::home::viewer::Viewer, models::FunctionList};

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

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focus: &Viewer,
    ) -> color_eyre::Result<()> {
        let title = Line::from(" Functions ");
        let border_color = match focus {
            Viewer::FunctionListViewer => Color::Magenta,
            _ => Color::Reset,
        };
        let block = Block::bordered()
            .title(title)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(border_color));

        let l = List::new(
            self.function_list
                .functions
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<String>>(),
        )
        .block(block)
        .highlight_symbol("-> ");

        frame.render_stateful_widget(l, area, &mut self.state);
        Ok(())
    }
}
