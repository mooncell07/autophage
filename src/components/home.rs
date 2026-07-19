mod disassembly_viewer;
mod function_list_viewer;

use crossterm::event::KeyCode;
use disassembly_viewer::DisassemblyViewer;
use function_list_viewer::FunctionListViewer;

use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::action::Action;

#[derive(Default, Debug)]
pub enum Viewer {
    FunctionListViewer,

    #[default]
    DisassemblyViewer,
}

#[derive(Default)]
pub struct Home {
    focused_viewer: Viewer,
    function_list_viewer: FunctionListViewer,
    disassembly_viewer: DisassemblyViewer,
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
        match key.code {
            KeyCode::Tab => {
                self.focused_viewer = match self.focused_viewer {
                    Viewer::FunctionListViewer => Viewer::DisassemblyViewer,
                    Viewer::DisassemblyViewer => Viewer::FunctionListViewer,
                };
            }

            KeyCode::Char('j') => match self.focused_viewer {
                Viewer::FunctionListViewer => {
                    self.function_list_viewer.state.select_next();
                }
                _ => {}
            },

            KeyCode::Char('k') => match self.focused_viewer {
                Viewer::FunctionListViewer => {
                    self.function_list_viewer.state.select_previous();
                }
                _ => {}
            },
            _ => {}
        };

        Ok(None)
    }
    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::ResultFunctionList(fl) => self.function_list_viewer.update(&fl),
            Action::ResultDisassembly(d) => self.disassembly_viewer.update(&d),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let body = Block::new();

        let body_area = body.inner(area);

        let [function_list_area, disassembly_area] = body_area.layout(&Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
        ]));

        let function_list_widget = self.function_list_viewer.get_widget();

        let disassembly_widget = self.disassembly_viewer.get_widget();
        frame.render_stateful_widget(
            &function_list_widget,
            function_list_area,
            &mut self.function_list_viewer.state,
        );

        frame.render_widget(&disassembly_widget, disassembly_area);

        Ok(())
    }
}
