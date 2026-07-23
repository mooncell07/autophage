use std::sync::Arc;

use super::viewer::Viewer;

use opaline::{Theme, load_by_name};
use ratatui::{
    Frame,
    layout::{self, Constraint, Rect},
    style::{Color, Style},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Cell, Paragraph, Row, Table},
};
use tui_scrollview::{ScrollView, ScrollViewState};

use crate::models::Disassembly;

pub struct DisassemblyViewer {
    disassembly: Arc<Disassembly>,
    pub state: ScrollViewState,
}

impl Default for DisassemblyViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl DisassemblyViewer {
    pub fn new() -> Self {
        return Self {
            disassembly: Disassembly::default().into(),
            state: ScrollViewState::new(),
        };
    }
    pub fn update(&mut self, disassembly: &Arc<Disassembly>) {
        self.disassembly = Arc::clone(disassembly);
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focus: &Viewer,
    ) -> color_eyre::Result<()> {
        let title = Line::from(" Disassembly ");

        let border_color = match focus {
            Viewer::DisassemblyViewer => Color::Magenta,
            _ => Color::Reset,
        };
        let block = Block::bordered()
            .title(title)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(border_color));

        let lines: Vec<Line> = self
            .disassembly
            .instructions
            .iter()
            .map(|instr| {
                Line::from(vec![
                    Span::raw(instr.address.to_string()),
                    Span::raw(
                        instr
                            .bytes
                            .as_bytes()
                            .chunks(2)
                            .map(|chunk| std::str::from_utf8(chunk).unwrap())
                            .collect::<Vec<&str>>()
                            .join(" ")
                            .to_string(),
                    ),
                    Span::raw(instr.mnemonic.to_string()),
                    Span::raw(instr.operands.join(",").to_string()),
                ])
            })
            .collect();

        let inner = block.inner(area);

        frame.render_widget(block, area);

        let para = Paragraph::new(lines);
        let virtual_size = layout::Size::new(inner.width, self.disassembly.count as u16);

        let mut scroll_view = ScrollView::new(virtual_size);
        let canvas_target = Rect::new(0, 0, virtual_size.width, virtual_size.height);
        scroll_view.render_widget(para, canvas_target);

        frame.render_stateful_widget(&scroll_view, inner, &mut self.state);
        Ok(())
    }
}
