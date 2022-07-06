use std::mem;

const BOARD_SIZE: usize = 8;

pub struct Coord {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub enum Colour {
    Black,
    White,
}

#[derive(Debug)]
pub enum Square {
    Empty,
    Bishop(Colour),
    King(Colour),
    Knight(Colour),
    Pawn(Colour),
    Queen(Colour),
    Rook(Colour),
}

#[derive(Debug)]
pub struct Board {
    layout: [[Square; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new(layout: [[Square; BOARD_SIZE]; BOARD_SIZE]) -> Board {
        Board { layout }
    }

    pub fn get_layout(&self) -> &[[Square; BOARD_SIZE]; BOARD_SIZE] {
        &self.layout
    }

    pub fn move_piece(&mut self, p_sq: &Coord, n_sq: &Coord) -> Result<(), &str> {
        if ((p_sq.x | n_sq.x) > self.layout[0].len() - 1)
            | ((p_sq.y | n_sq.y) > self.layout.len() - 1)
        {
            return Err("Coordinates entered are out of bounds!");
        }
        let square = mem::replace(
            &mut self.layout[self.layout.len() - 1 - p_sq.y][p_sq.x],
            Square::Empty,
        );
        self.layout[self.layout.len() - 1 - n_sq.y][n_sq.x] = square;
        Ok(())
    }
}
