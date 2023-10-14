use crate::state;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::StatefulWidget,
    style,
};

pub struct ChessWidget {
}

impl ChessWidget {
    pub fn new() -> Self {
        Self {}
    }
}


impl StatefulWidget for ChessWidget {
    type State = state::State;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.left() + 12 > area.right() {
            return;
        }
        if area.top() + 12 > area.bottom() {
            return;
        }
        for y in 0..8 {
            for x in 0..8 {
                let bg = {
                    if (x+y) % 2 == 0 {
                        style::Color::Rgb(184,139,74)
                    } else {
                        style::Color::Rgb(227,193,111)
                    }
                };
                buf.get_mut(x, y).set_bg(bg);
            }
        }
        for w in state.board.whites.iter() {
            let (x, y) = w.pos().to_x_y();
            let cell = buf.get_mut(x, y);
            cell.set_fg(style::Color::White);
            cell.set_symbol(w.symbol());
        }
        for b in state.board.blacks.iter() {
            let (x, y) = b.pos().to_x_y();
            let cell = buf.get_mut(x, y);
            cell.set_fg(style::Color::Black);
            cell.set_symbol(b.symbol());
        }
    }
}
