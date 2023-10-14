use std::vec::Vec;

pub enum Color {
    White,
    Black
}

// u16 for simple compatibility with ratatui
#[derive(Clone, Copy)]
pub struct Pos {
    pub file: u16,
    pub rank: u16,
}

impl Pos {
    pub fn new(file: u16, rank: u16) -> Self {
        Pos{file, rank}
    }

    pub fn to_x_y(&self) -> (u16, u16) {
        (self.file-1, 8-self.rank)
    }
}

pub enum Figure {
    Pawn(Pos),
    Rook(Pos),
    Knight(Pos),
    Bishop(Pos),
    Queen(Pos),
    King(Pos)
}

use crate::state::Figure::*;

impl Figure {
    pub fn symbol(&self) -> &str {
        match self {
            Pawn(_) => "♟︎",
            Rook(_) => "♜",
            Knight(_) => "♞",
            Bishop(_) => "♝",
            Queen(_) => "♛",
            King(_) => "♚",
        }
    }

    pub fn pos(&self) -> Pos {
        match self {
            Pawn(pos) => *pos,
            Rook(pos) => *pos,
            Knight(pos) => *pos,
            Bishop(pos) => *pos,
            Queen(pos) => *pos,
            King(pos) => *pos,
        }
    }
}

pub struct Board {
    pub whites: Vec<Figure>,
    pub blacks: Vec<Figure>
}

impl Board {
    pub fn new() -> Self {
        let mut whites = vec![];
        let mut blacks = vec![];
        // Pawns
        for r in 1..=8 {
            whites.push(Pawn(Pos::new(r, 2)));
            blacks.push(Pawn(Pos::new(r, 7)));
        }
        // Rooks
        whites.push(Rook(Pos::new(1, 1)));
        whites.push(Rook(Pos::new(8, 1)));
        blacks.push(Rook(Pos::new(1, 8)));
        blacks.push(Rook(Pos::new(8, 8)));
        // Knights
        whites.push(Knight(Pos::new(2, 1)));
        whites.push(Knight(Pos::new(7, 1)));
        blacks.push(Knight(Pos::new(2, 8)));
        blacks.push(Knight(Pos::new(7, 8)));
        // Bishops
        whites.push(Bishop(Pos::new(3, 1)));
        whites.push(Bishop(Pos::new(6, 1)));
        blacks.push(Bishop(Pos::new(3, 8)));
        blacks.push(Bishop(Pos::new(6, 8)));
        // Queens
        whites.push(Queen(Pos::new(4, 1)));
        blacks.push(Queen(Pos::new(4, 8)));
        // Kings
        whites.push(King(Pos::new(5, 1)));
        blacks.push(King(Pos::new(5, 8)));
        Board {whites, blacks}
    }
}

pub struct State {
    pub board: Board,
    pub turn: Color,
}

impl State {
    pub fn new() -> Self {
        State{board: Board::new(), turn: Color::White}
    }
}
