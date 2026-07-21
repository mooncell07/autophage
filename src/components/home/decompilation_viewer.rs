use std::sync::Arc;

use crossterm::event::KeyEvent;
use edtui::{
    EditorEventHandler, EditorState, EditorTheme, EditorView, LineNumbers, Lines, SyntaxHighlighter,
};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols::border,
    text::Line,
    widgets::Block,
};
use syntect::highlighting::Theme;
use syntect::highlighting::ThemeSet;

use super::viewer::Viewer;
use crate::models::Decompilation;

pub struct DecompilationViewer {
    decompilation: Arc<Decompilation>,
    state: EditorState,
    event_handler: EditorEventHandler,
    syntax_theme: Theme,
}

impl Default for DecompilationViewer {
    fn default() -> Self {
        Self::new()
    }
}

impl DecompilationViewer {
    pub fn new() -> Self {
        let syntax_theme = ThemeSet::get_theme("theme/template.tmTheme")
            .unwrap()
            .clone();

        return Self {
            decompilation: Decompilation::default().into(),
            state: EditorState::default(),
            event_handler: EditorEventHandler::default(),
            syntax_theme,
        };
    }
    pub fn update(&mut self, decompilation: &Arc<Decompilation>) {
        let decompilation = Arc::clone(decompilation);
        self.state.lines = Lines::from(decompilation.code.clone());
        self.decompilation = decompilation;
    }

    pub fn handle_editor_key_events(&mut self, key_event: KeyEvent) {
        self.event_handler.on_key_event(key_event, &mut self.state);
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        focus: &Viewer,
    ) -> color_eyre::Result<()> {
        let title = Line::from(" Decompilation ");

        let border_color = match focus {
            Viewer::DecompilationViewer => Color::Magenta,
            _ => Color::Reset,
        };

        let block = Block::bordered()
            .title(title)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(border_color));
        let theme = EditorTheme::default()
            .block(block)
            .base(Style::default().bg(Color::Reset))
            .hide_status_line()
            .line_numbers_style(Style::default());

        let syntax_highlighter = SyntaxHighlighter::new("dracula", "c")
            .unwrap()
            .custom_theme(self.syntax_theme.clone());
        frame.render_widget(
            EditorView::new(&mut self.state)
                .syntax_highlighter(Some(syntax_highlighter))
                .line_numbers(LineNumbers::Relative)
                .theme(theme),
            area,
        );

        Ok(())
    }
}
