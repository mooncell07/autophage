use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Cell, Row, Table, Widget},
};

use crate::adapter::Adapter;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub address: String,
    pub bytes: String,
    pub mnemonic: String,
}

pub struct App {
    adapter: Adapter,
    disasm: Vec<Instruction>,
    exit: bool,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Disassembly Viewer ".bold());
        let block = Block::bordered().title(title).border_set(border::PLAIN);

        let rows: Vec<Row> = self
            .disasm
            .iter()
            .map(|instr| {
                Row::new(vec![
                    Cell::from(instr.address.as_str()),
                    Cell::from(instr.bytes.as_str()),
                    Cell::from(instr.mnemonic.as_str()),
                ])
            })
            .collect();

        let max_address_len = self
            .disasm
            .iter()
            .map(|instr| instr.address.len())
            .max()
            .unwrap_or(15);

        let widths = [
            Constraint::Length(max_address_len as u16),
            Constraint::Length(15),
            Constraint::Min(20),
        ];

        Table::new(rows, widths)
            .block(block)
            .column_spacing(4)
            .render(area, buf);
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Down => self.run_disasm(),
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
        self.adapter.close();
    }

    fn run_disasm(&mut self) {
        let res = self.adapter.get_disassembly("140001000", 10).unwrap();
        if let Some(instrs) = res["instructions"].as_array() {
            for instr in instrs {
                let address = instr["address"].as_str().unwrap().to_string();
                let bytes = instr["bytes"].as_str().unwrap().to_string();
                let mnemonic = instr["mnemonic"].as_str().unwrap().to_string();
                self.disasm.push(Instruction {
                    address,
                    bytes,
                    mnemonic,
                });
            }
        }
    }
}

pub fn ui_main(adapter: Adapter) {
    ratatui::run(|terminal| {
        App {
            adapter,
            disasm: Vec::new(),
            exit: false,
        }
        .run(terminal)
    })
    .unwrap()
}
