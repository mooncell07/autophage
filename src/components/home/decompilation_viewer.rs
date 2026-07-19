use std::{collections::LinkedList, sync::Arc};

use ratatui::{
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::models::Decompilation;

pub struct DecompilationViewer {
    decompilation: Arc<Decompilation>,
}

impl Default for DecompilationViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl DecompilationViewer {
    pub fn new() -> Self {
        return Self {
            decompilation: Decompilation::default().into(),
        };
    }
    pub fn update(&mut self, decompilation: &Arc<Decompilation>) {
        self.decompilation = Arc::clone(decompilation);
    }

    pub fn get_widget(&self) -> Paragraph<'_> {
        let title = Line::from(" Decompilation ");
        let block = Block::bordered().title(title).border_set(border::PLAIN);
        let lines: Vec<Line<'_>> = self
            .decompilation
            .code
            .split("\r\n")
            .map(|line| Line::from(String::from(line)))
            .collect::<Vec<Line>>()
            .into();
        Paragraph::new(lines).block(block)
    }
}
