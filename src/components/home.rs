mod decompilation_viewer;
mod disassembly_viewer;
mod function_list_viewer;

use crossterm::event::{KeyCode, KeyModifiers};
use decompilation_viewer::DecompilationViewer;
use disassembly_viewer::DisassemblyViewer;
use function_list_viewer::FunctionListViewer;

use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;

#[derive(Default, Debug)]
pub enum Viewer {
    FunctionListViewer,
    DecompilationViewer,

    #[default]
    DisassemblyViewer,
}

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
        let mut action: Option<Action> = None;

        let keycode = key.code;
        let modifier = key.modifiers;

        match (keycode, modifier) {
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                self.focused_viewer = match self.focused_viewer {
                    Viewer::DecompilationViewer => Viewer::DisassemblyViewer,
                    Viewer::DisassemblyViewer => Viewer::FunctionListViewer,
                    Viewer::FunctionListViewer => Viewer::FunctionListViewer,
                };
            }

            (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                self.focused_viewer = match self.focused_viewer {
                    Viewer::FunctionListViewer => Viewer::DisassemblyViewer,
                    Viewer::DisassemblyViewer => Viewer::DecompilationViewer,
                    Viewer::DecompilationViewer => Viewer::DecompilationViewer,
                }
            }

            (KeyCode::Char('j'), KeyModifiers::CONTROL) => {}
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {}
            _ => {}
        }

        match self.focused_viewer {
            Viewer::DecompilationViewer => {
                self.decompilation_viewer.handle_editor_key_events(key);
            }

            Viewer::FunctionListViewer => match (keycode, modifier) {
                (KeyCode::Char('j'), KeyModifiers::NONE) => {
                    self.function_list_viewer.state.select_next();
                }
                (KeyCode::Char('k'), KeyModifiers::NONE) => {
                    self.function_list_viewer.state.select_previous();
                }
                (KeyCode::Enter, KeyModifiers::NONE) => {
                    let index = self.function_list_viewer.state.selected_mut().unwrap();
                    let address = self.function_list_viewer.get_function_address(index);
                    action = Some(Action::RequestDecompilation(Some(address)));
                }
                _ => {}
            },

            Viewer::DisassemblyViewer => {}
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

        let _ = self.function_list_viewer.render(frame, function_list_area);
        let _ = self.disassembly_viewer.render(frame, disassembly_area);
        let _ = self.decompilation_viewer.render(frame, remaining_area);

        Ok(())
    }
}
