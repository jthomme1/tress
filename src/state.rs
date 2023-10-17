use std::vec::Vec;
use std::result::Result;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn other(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

use crate::state::Color::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Dir {
    pub fn get_diagonals(&self) -> Vec<Dir> {
        match self {
            Left => vec![LeftUp, LeftDown],
            Right => vec![RightUp, RightDown],
            Up => vec![LeftUp, RightUp],
            Down => vec![Left, RightDown],
            _ => panic!("undefined"),
        }
    }
}

use crate::state::Dir::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub file: usize,
    pub rank: usize,
}

impl Pos {
    pub fn new(file: usize, rank: usize) -> Result<Pos, ()> {
        if file <= 8 && file >= 1 && rank <= 8 && rank >= 1 {
            Ok(Pos{file, rank})
        } else {
            Err(())
        }
    }

    pub fn to_ratatui(&self) -> (u16, u16) {
        (u16::try_from(self.file-1).unwrap(), u16::try_from(8-self.rank).unwrap())
    }

    pub fn to_array(&self) -> (usize, usize) {
        (self.file-1, 8-self.rank)
    }

    pub fn advance(&self, color: Color) -> Result<Pos, ()> {
        match color {
            White => self.move_dir(Up),
            Black => self.move_dir(Down),
        }
    }

    pub fn move_dir(&self, dir: Dir) -> Result<Pos, ()> {
        match dir {
            Up => Pos::new(self.file, self.rank+1),
            Down => Pos::new(self.file, self.rank-1),
            Left => Pos::new(self.file-1, self.rank),
            Right => Pos::new(self.file+1, self.rank),
            LeftUp => Pos::new(self.file-1, self.rank+1),
            LeftDown => Pos::new(self.file-1, self.rank-1),
            RightUp => Pos::new(self.file+1, self.rank+1),
            RightDown => Pos::new(self.file+1, self.rank-1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Figure {
    Pawn(Color, bool),
    Rook(Color, bool),
    Knight(Color, bool),
    Bishop(Color, bool),
    Queen(Color, bool),
    King(Color, bool)
}

use crate::state::Figure::*;

impl Figure {
    pub fn symbol(&self) -> &str {
        match self {
            Pawn(_,_) => "♟︎",
            Rook(_,_) => "♜",
            Knight(_,_) => "♞",
            Bishop(_,_) => "♝",
            Queen(_,_) => "♛",
            King(_,_) => "♚",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Pawn(c,_) => *c,
            Rook(c,_) => *c,
            Knight(c,_) => *c,
            Bishop(c,_) => *c,
            Queen(c,_) => *c,
            King(c,_) => *c,
        }
    }

    pub fn has_moved(&self) -> bool {
        match self {
            Pawn(_,m) => *m,
            Rook(_,m) => *m,
            Knight(_,m) => *m,
            Bishop(_,m) => *m,
            Queen(_,m) => *m,
            King(_,m) => *m,
        }
    }

    pub fn set_moved_to(&mut self, val: bool) {
        match self {
            Pawn(_,m) => *m = val,
            Rook(_,m) => *m = val,
            Knight(_,m) => *m = val,
            Bishop(_,m) => *m = val,
            Queen(_,m) => *m = val,
            King(_,m) => *m = val,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Move {
    // All checking whether a move is possible is not done here.
    // A Normal move means that we just move a single figure, not taking
    // a figure of the opponent.
    // Associated values represent the old position of the piece and the new one
    // as well as the has_moved value before the move
    Normal(Pos, Pos, bool),
    // An attacking move from the first position to the second position taking an
    // opponents figure as well as the has_moved value before the move
    Take(Pos, Pos, Figure, bool),
    // The Castle move.
    // Associated values: King old position, Rook old position, King new position, Rook new
    // position. We don't have a has_moved value as this has to be false before for both.
    Castle(Pos, Pos, Pos, Pos),
    // The Promotion of a Pawn to a new Figure.
    // Associated values: old position Pawn, new position, Figure replacing the Pawn
    Promote(Pos, Pos, Figure)
}

impl Move {
    pub fn from_pos(&self) -> Pos {
        match self {
            Normal(p,_,_) => *p,
            Take(p,_,_,_) => *p,
            Castle(p,_,_,_) => *p,
            Promote(p,_,_) => *p,
        }
    }

    pub fn to_pos(&self) -> Pos {
        match self {
            Normal(_,p,_) => *p,
            Take(_,p,_,_) => *p,
            Castle(_,_,p,_) => *p,
            Promote(_,p,_) => *p,
        }
    }

    pub fn has_moved_before(&self) -> bool {
        match self {
            Normal(_,_,m) => *m,
            Take(_,_,_,m) => *m,
            Castle(_,_,_,_) => false,
            Promote(_,_,_) => true,
        }
    }
}

use crate::state::Move::*;

pub struct Board {
    pub fields: [[Option<Figure>; 8]; 8]
}

impl Board {
    pub fn new() -> Self {
        let fields: [[Option<Figure>; 8]; 8] = [[None; 8]; 8];
        let mut board = Board {fields};
        // Pawns
        for r in 1..=8 {
            board.add_figure(Pos::new(r, 2).unwrap(), Pawn(White, false));
            board.add_figure(Pos::new(r, 7).unwrap(), Pawn(Black, false));
        }
        // Rooks
        board.add_figure(Pos::new(1, 1).unwrap(), Rook(White, false));
        board.add_figure(Pos::new(8, 1).unwrap(), Rook(White, false));
        board.add_figure(Pos::new(1, 8).unwrap(), Rook(Black, false));
        board.add_figure(Pos::new(8, 8).unwrap(), Rook(Black, false));
        // Knights
        board.add_figure(Pos::new(2, 1).unwrap(), Knight(White, false));
        board.add_figure(Pos::new(7, 1).unwrap(), Knight(White, false));
        board.add_figure(Pos::new(2, 8).unwrap(), Knight(Black, false));
        board.add_figure(Pos::new(7, 8).unwrap(), Knight(Black, false));
        // Bishops
        board.add_figure(Pos::new(3, 1).unwrap(), Bishop(White, false));
        board.add_figure(Pos::new(6, 1).unwrap(), Bishop(White, false));
        board.add_figure(Pos::new(3, 8).unwrap(), Bishop(Black, false));
        board.add_figure(Pos::new(6, 8).unwrap(), Bishop(Black, false));
        // Queens
        board.add_figure(Pos::new(4, 1).unwrap(), Queen(White, false));
        board.add_figure(Pos::new(4, 8).unwrap(), Queen(Black, false));
        // Kings
        board.add_figure(Pos::new(5, 1).unwrap(), King(White, false));
        board.add_figure(Pos::new(5, 8).unwrap(), King(Black, false));
        board
    }

    pub fn fields_get_mut(&mut self, pos: Pos) -> &mut Option<Figure>{
        let (x, y) = pos.to_array();
        &mut self.fields[x][y]
    }

    pub fn fields_get(&self, pos: Pos) -> Option<Figure> {
        let (x, y) = pos.to_array();
        self.fields[x][y]
    }

    pub fn add_figure(&mut self, pos: Pos, figure: Figure) {
        *self.fields_get_mut(pos) = Some(figure);
    }

    pub fn remove_figure(&mut self, pos: Pos) -> Option<Figure> {
        self.fields_get_mut(pos).take()
    }

    pub fn move_figure(&mut self,
                          old_pos: Pos,
                          new_pos: Pos,
                          has_moved_val: bool) {
        let mut moving_figure = self.remove_figure(old_pos).unwrap();
        moving_figure.set_moved_to(has_moved_val);
        self.add_figure(new_pos, moving_figure);
    }

    pub fn do_move(&mut self, mv: Move) {
        // we do not check the Move for validity, this logic is done in ChessState
        match mv {
            Normal(old_pos, new_pos, _) => self.move_figure(old_pos, new_pos, true),
            Take(old_pos, new_pos, _, _) => self.move_figure(old_pos, new_pos, true),
            Castle(k_old_pos, r_old_pos, k_new_pos, r_new_pos) => {
                self.move_figure(k_old_pos, k_new_pos, true);
                self.move_figure(r_old_pos, r_new_pos, true);
            },
            Promote(old_pos, new_pos, figure) => {
                self.remove_figure(old_pos);
                self.add_figure(new_pos, figure);
            },
        }
    }

    pub fn undo_move(&mut self, mv: Move) {
        match mv {
            Normal(old_pos, new_pos, m) => self.move_figure(new_pos, old_pos, m),
            Take(old_pos, new_pos, taken, m) => {
                self.move_figure(new_pos, old_pos, m);
                self.add_figure(new_pos, taken);
            },
            Castle(k_old_pos, r_old_pos, k_new_pos, r_new_pos) => {
                self.move_figure(k_new_pos, k_old_pos, false);
                self.move_figure(r_new_pos, r_old_pos, false);
            },
            Promote(old_pos, new_pos, figure) => {
                self.remove_figure(new_pos);
                self.add_figure(old_pos, Pawn(figure.color(), true));
            },
        }
    }
}

pub struct ChessState {
    pub board: Board,
    pub turn: Color,
    pub mate: bool,
}

impl ChessState {
    pub fn new() -> Self {
        ChessState{board: Board::new(), turn: White, mate: false}
    }

    pub fn player_is_in_check(&mut self) -> bool {
        self.check_for_mate(self.turn)
    }

    pub fn player_has_moves(&mut self) -> bool {
        for file in 1..=8 {
            for rank in 1..=8 {
                let pos = Pos::new(file, rank).unwrap();
                if let Some(figure) = self.board.fields_get(pos) {
                    if figure.color() == self.turn
                        && self.possible_moves(pos, true).len() != 0 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn check_for_mate(&mut self, color: Color) -> bool {
        let enemy = color.other();
        for file in 1..=8 {
            for rank in 1..=8 {
                let pos = Pos::new(file, rank).unwrap();
                if let Some(figure) = self.board.fields_get(pos) {
                    if figure.color() == enemy {
                        if self.possible_moves(pos, false)
                            .iter()
                            .position(|mv| {
                                if let Take(_, _, attacked_fig, _) = mv {
                                    return matches!(attacked_fig, King(_, _));
                                }
                                false
                            })
                            .is_some() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn check_move_for_mate(&mut self, mv: Move) -> bool {
        // temporarily move the piece(s)...
        self.board.do_move(mv);
        // ...to check if there would be a check mate...
        let result = self.check_for_mate(self.turn);
        // ...and move the piece(s) back
        self.board.undo_move(mv);
        !result
    }

    fn possible_moves(&mut self, pos: Pos, check_for_mate: bool) -> Vec<Move> {
        let mut moves = vec![];
        let figure = self.board.fields_get(pos).unwrap();
        let color = figure.color();
        let has_moved = figure.has_moved();
        let mut push_moves_line = |dirs: Vec<Dir>, single: bool| {
                let mut cur_pos = pos;
                for dir in dirs {
                    while let Ok(next_pos) = cur_pos.move_dir(dir) {
                        match self.board.fields_get(next_pos) {
                            Some(fig) => {
                                if fig.color() != color {
                                    moves.push(Take(pos, next_pos, fig, has_moved));
                                }
                                break;
                            },
                            None => moves.push(Normal(pos, next_pos, has_moved)),
                        };
                        if single {
                            break;
                        }
                        cur_pos = next_pos;
                    }
                    cur_pos = pos;
                }
        };
        match figure {
            Pawn(_,_) => {
                    let forward_pos = pos.advance(color).unwrap();
                    if self.board.fields_get(forward_pos).is_none() {
                        moves.push(Normal(pos,
                                          forward_pos,
                                          has_moved));
                        if !has_moved {
                            let double_forward_pos = forward_pos.advance(color)
                                .unwrap();
                            if self.board.fields_get(double_forward_pos).is_none() {
                                moves.push(Normal(pos,
                                                  double_forward_pos,
                                                  has_moved));
                            }
                        }
                    }
                    for dir in [Left, Right] {
                        if let Ok(attack_pos) = forward_pos.move_dir(dir) {
                            let new_field = self.board.fields_get(attack_pos);
                            if let Some(attacked_fig) = new_field {
                                if attacked_fig.color() != color {
                                    moves.push(Take(pos,
                                                    attack_pos,
                                                    attacked_fig,
                                                    has_moved));
                                }
                            }
                        }
                    }
                },
            Rook(_,_) => push_moves_line(vec![Left, Right, Up, Down], false),
            Knight(_,_) => {
                let dirs = [Left, Right, Up, Down];
                for dir in dirs {
                    if let Ok(one_moved_pos) = pos.move_dir(dir) {
                        for diag in dir.get_diagonals() {
                            if let Ok(jump_pos) = one_moved_pos.move_dir(diag) {
                                match self.board.fields_get(jump_pos) {
                                    Some(attacked_fig) => {
                                        if attacked_fig.color() != color {
                                            moves.push(Take(pos,
                                                            jump_pos,
                                                            attacked_fig,
                                                            has_moved));
                                        }
                                    },
                                    None => moves.push(Normal(pos, jump_pos, has_moved)),
                                }
                            }
                        }
                    }
                }
            },
            Bishop(_,_) => push_moves_line(vec![LeftUp, LeftDown, RightUp, RightDown], false),
            Queen(_,_) => push_moves_line(vec![Left, Right, Up, Down,
                                          LeftUp, LeftDown, RightUp, RightDown], false),
            King(_,_) => push_moves_line(vec![Left, Right, Up, Down,
                                          LeftUp, LeftDown, RightUp, RightDown], true),
        };
        if check_for_mate {
            moves.retain(|mv| self.check_move_for_mate(*mv));
        }
        moves
    }

    pub fn move_checked(&mut self, pos: Pos, new_pos: Pos) -> Result<(), ()> {
        match self.board.fields_get(pos) {
            None => return Err(()),
            Some(fig) => if fig.color() != self.turn {return Err(());},
        }
        let possible_moves = self.possible_moves(pos, true);
        let mv_ind = possible_moves
            .iter()
            .position(|mv| {
                pos == mv.from_pos() && new_pos == mv.to_pos()
            })
            .ok_or(())?;
        self.board.do_move(possible_moves[mv_ind]);
        self.turn = self.turn.other();
        Ok(())
    }
}
