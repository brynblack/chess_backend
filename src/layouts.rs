use crate::board::{Colour, Square};

pub struct Layouts;

impl Layouts {
    pub fn standard() -> [[Square; 8]; 8] {
        [
            [
                Square::Rook(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Queen(Colour::Black),
                Square::King(Colour::Black),
                Square::Bishop(Colour::Black),
                Square::Knight(Colour::Black),
                Square::Rook(Colour::Black),
            ],
            [
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
                Square::Pawn(Colour::Black),
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            [
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
                Square::Pawn(Colour::White),
            ],
            [
                Square::Rook(Colour::White),
                Square::Knight(Colour::White),
                Square::Bishop(Colour::White),
                Square::Queen(Colour::White),
                Square::King(Colour::White),
                Square::Bishop(Colour::White),
                Square::Knight(Colour::White),
                Square::Rook(Colour::White),
            ],
        ]
    }
}
