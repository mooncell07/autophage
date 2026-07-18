use std::sync::Arc;

use opaline::{Theme, load_by_name};
use ratatui::{
    layout::Constraint,
    style::Modifier,
    symbols::border,
    text::Line,
    widgets::{Block, Cell, List, Row, Table},
};

use crate::models::Disassembly;

pub struct DisassemblyViewer {
    theme: Theme,
    disassembly: Arc<Disassembly>,
}

impl Default for DisassemblyViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl DisassemblyViewer {
    pub fn new() -> Self {
        return Self {
            theme: load_by_name("rose-pine").unwrap(),
            disassembly: Disassembly::default().into(),
        };
    }
    pub fn update(&mut self, disassembly: &Arc<Disassembly>) {
        self.disassembly = Arc::clone(disassembly);
    }

    pub fn get_widget(&self) -> Table<'_> {
        let title = Line::from(" Disassembly Viewer ");
        let block = Block::bordered().title(title).border_set(border::PLAIN);

        let rows: Vec<Row> = self
            .disassembly
            .instructions
            .iter()
            .map(|instr| {
                Row::new(vec![
                    Cell::from(self.theme.span("primary", instr.address.as_str())),
                    Cell::from(
                        self.theme.span(
                            "line_number",
                            instr
                                .bytes
                                .as_bytes()
                                .chunks(2)
                                .map(|chunk| std::str::from_utf8(chunk).unwrap())
                                .collect::<Vec<&str>>()
                                .join(" "),
                        ),
                    ),
                    Cell::from(self.theme.span("info_style", instr.mnemonic.as_str())),
                    Cell::from(self.theme.span("warning_style", instr.operands.join(","))),
                ])
            })
            .collect();

        let max_address_len = self
            .disassembly
            .instructions
            .iter()
            .map(|instr| instr.address.len())
            .max()
            .unwrap_or(15);

        let widths = [
            Constraint::Length(max_address_len as u16),
            Constraint::Length(15),
            Constraint::Length(5),
            Constraint::Length(5),
        ];
        Table::new(rows, widths).block(block).column_spacing(4)
    }
}
