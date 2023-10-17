use crate::render::{ChessWidget, ChessWidgetState};
use crate::state::Dir;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
};
use std::io::{stderr, Result};

pub mod render;
pub mod state;

fn main() -> Result<()> {
    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut state = ChessWidgetState::new();

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_stateful_widget(ChessWidget::new(), area, &mut state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('j') {
                    state.move_cursor(Dir::Down);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('k') {
                    state.move_cursor(Dir::Up);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('h') {
                    state.move_cursor(Dir::Left);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('l') {
                    state.move_cursor(Dir::Right);
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char(' ') {
                    state.toggle_select();
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                    state.quit_select();
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
