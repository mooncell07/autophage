mod decompilation_viewer;
mod disassembly_viewer;
mod function_list_viewer;
mod viewer;

use crossterm::event::{KeyCode, KeyModifiers};
use decompilation_viewer::DecompilationViewer;
use disassembly_viewer::DisassemblyViewer;
use function_list_viewer::FunctionListViewer;
use viewer::Viewer;

use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;

#[derive(Default)]
pub struct Home {
    focused_viewer: Viewer,
    function_list_viewer: FunctionListViewer,
    disassembly_viewer: DisassemblyViewer,
    decompilation_viewer: DecompilationViewer,
    command_tx: Option<UnboundedSender<Action>>,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::Result<Option<Action>> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('h') | KeyCode::Left, KeyModifiers::CONTROL) => {
                self.focused_viewer = Viewer::move_left(&self.focused_viewer);
            }

            (KeyCode::Char('l') | KeyCode::Right, KeyModifiers::CONTROL) => {
                self.focused_viewer = Viewer::move_right(&self.focused_viewer);
            }

            (KeyCode::Char('j') | KeyCode::Down, KeyModifiers::CONTROL) => {}
            (KeyCode::Char('k') | KeyCode::Up, KeyModifiers::CONTROL) => {}
            _ => {}
        }

        let action: Option<Action> = match self.focused_viewer {
            Viewer::DecompilationViewer => self.decompilation_viewer.handle_key_events(key),

            Viewer::FunctionListViewer => self.function_list_viewer.handle_key_events(key),

            Viewer::DisassemblyViewer => self.disassembly_viewer.handle_key_events(key),
        };
        Ok(action)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::ResultFunctionList(fl) => self.function_list_viewer.update(&fl),
            Action::ResultDisassembly(dism) => self.disassembly_viewer.update(&dism),
            Action::ResultDecompilation(decomp) => self.decompilation_viewer.update(&decomp),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let body = Block::new();

        let body_area = body.inner(area);

        let [function_list_area, disassembly_area, remaining_area] =
            body_area.layout(&Layout::horizontal([
                Constraint::Percentage(15),
                Constraint::Percentage(50),
                Constraint::Percentage(35),
            ]));

        let _ = self
            .function_list_viewer
            .render(frame, function_list_area, &self.focused_viewer);
        let _ = self
            .disassembly_viewer
            .render(frame, disassembly_area, &self.focused_viewer);
        let _ = self
            .decompilation_viewer
            .render(frame, remaining_area, &self.focused_viewer);

        Ok(())
    }
}
