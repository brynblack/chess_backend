use std::mem;

const BOARD_SIZE: usize = 8;

pub struct Position {
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

    pub fn move_piece(&mut self, old_pos: &Position, new_pos: &Position) -> Result<(), &str> {
        if ((old_pos.x | new_pos.x) > BOARD_SIZE - 1) | ((old_pos.y | new_pos.y) > BOARD_SIZE - 1) {
            return Err("Coordinates entered are out of bounds!");
        }
        let square = mem::replace(
            &mut self.layout[BOARD_SIZE - 1 - old_pos.y][old_pos.x],
            Square::Empty,
        );
        self.layout[BOARD_SIZE - 1 - new_pos.y][new_pos.x] = square;
        Ok(())
    }
}
