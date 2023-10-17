use crate::state::{self, Dir, Pos, ChessState};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::StatefulWidget,
    style,
    prelude::Style,
};

pub struct ChessWidget {}

impl ChessWidget {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ChessWidgetState {
    chess_state: ChessState,
    cursor: Pos,
    selected: Option<Pos>,
}

impl ChessWidgetState {
    pub fn new() -> Self {
        Self {chess_state: ChessState::new(),
              cursor: Pos::new(5, 2).unwrap(),
              selected: None}
    }

    pub fn move_cursor(&mut self, dir: Dir) {
        self.cursor = match self.cursor.move_dir(dir) {
            Ok(pos) => pos,
            Err(_) => self.cursor,
        }
    }

    pub fn toggle_select(&mut self) {
        if !self.chess_state.player_has_moves() {
            return;
        }
        if let Some(sel) = self.selected {
            if self.cursor != sel {
                match self.chess_state.move_checked(sel, self.cursor) {
                    Ok(()) => (),
                    Err(()) => (),
                }
            }
            self.selected = None;
        } else {
            self.selected = Some(self.cursor);
        }
    }

    pub fn quit_select(&mut self) {
        self.selected = None;
    }
}


impl StatefulWidget for ChessWidget {
    type State = ChessWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let chess_state = &mut state.chess_state;
        if area.left() + 8 > area.right() {
            return;
        }
        if area.top() + 8 > area.bottom() {
            return;
        }
        if !chess_state.player_has_moves() {
            let mut message: String = "".to_owned();
            let color_s = match chess_state.turn {
                state::Color::White => "White",
                state::Color::Black => "Black",
            };
            if chess_state.player_is_in_check() {
                message.push_str(color_s);
                message.push_str(" lost!");
            } else {
                message.push_str("It's a draw!");
            };
            message.push_str(" Press 'q' to exit.");
            for x in area.left()..area.right() {
                for y in area.top()..area.bottom() {
                    buf.get_mut(x, y).reset();
                }
            }
            buf.set_string(0, 0, message, Style::default());
            return;
        }
        for y in 0..8 {
            for x in 0..8 {
                let bg = {
                    if (x+y) % 2 == 1 {
                        style::Color::Rgb(184,139,74)
                    } else {
                        style::Color::Rgb(227,193,111)
                    }
                };
                let cell = buf.get_mut(x, y);
                cell.set_bg(bg);
                if let Some(fig) = chess_state.board.fields[x as usize][y as usize] {
                    let fg = match fig.color() {
                        state::Color::White => style::Color::White,
                        state::Color::Black => style::Color::Black,
                    };
                    cell.set_fg(fg);
                    cell.set_symbol(fig.symbol());
                }
            }
        }
        let (cursor_x, cursor_y) = state.cursor.to_ratatui();
        let cursor_color = match state.chess_state.turn {
            state::Color::White => style::Color::Blue,
            state::Color::Black => style::Color::Red,
        };
        buf.get_mut(cursor_x, cursor_y).set_bg(cursor_color);
        if let Some(sel) = state.selected {
            let (sel_x, sel_y) = sel.to_ratatui();
            buf.get_mut(sel_x, sel_y).set_bg(style::Color::Green);
        }
    }
}
