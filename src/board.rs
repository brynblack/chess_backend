// chess_backend - A library for creating and managing a chessboard, written in Rust.
// Copyright (C) 2022  Brynley Llewellyn-Roux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::mem;

/// A position on the chessboard.
#[derive(Debug)]
pub struct Position {
    /// The x-coordinate.
    pub x: usize,
    /// The y-coordinate.
    pub y: usize,
}

/// A square on the chessboard.
#[derive(Debug)]
pub enum Square {
    /// An empty square.
    Empty,
    /// A square with a piece on it.
    Piece {
        /// The colour of the piece.
        piece_colour: PieceColour,
        /// The type of the piece.
        piece_type: PieceType,
    },
}

/// The colour of a piece.
#[derive(Debug, PartialEq)]
pub enum PieceColour {
    /// The colour black.
    Black,
    /// The colour white.
    White,
}

/// The type of a piece.
#[derive(Debug)]
pub enum PieceType {
    /// A Bishop.
    Bishop,
    /// A King.
    King,
    /// A Knight.
    Knight,
    /// A Pawn.
    Pawn,
    /// A Queen.
    Queen,
    /// A Rook.
    Rook,
}

impl Square {
    /// Returns an optional value containing a reference to the colour of the piece.
    pub fn get_colour(&self) -> Option<&PieceColour> {
        match self {
            Square::Empty => None,
            Square::Piece { piece_colour, .. } => Some(piece_colour),
        }
    }
}

/// A chessboard.
pub struct Board {
    layout: Vec<Vec<Square>>,
    player: PieceColour,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            layout: Layouts::standard(),
            player: PieceColour::White,
        }
    }
}

impl Board {
    /// Creates a new chessboard with the given layout and the default player specified.
    pub fn new(layout: Vec<Vec<Square>>, player: PieceColour) -> Self {
        Self { layout, player }
    }

    /// Returns a reference to the current chessboard layout.
    pub fn get_layout(&self) -> &Vec<Vec<Square>> {
        &self.layout
    }

    /// Returns a reference to the current player.
    pub fn get_player(&self) -> &PieceColour {
        &self.player
    }

    /// Moves a piece on the chessboard from one position to another.
    pub fn move_piece(&mut self, old_pos: &Position, new_pos: &Position) -> Result<(), &str> {
        if let Err(err) = self.is_valid_move(old_pos, new_pos) {
            return Err(err);
        }
        let moved_piece = mem::replace(&mut self.layout[old_pos.y][old_pos.x], Square::Empty);
        self.layout[new_pos.y][new_pos.x] = moved_piece;
        self.next_turn();
        Ok(())
    }

    fn is_valid_move<'a>(&self, old_pos: &Position, new_pos: &Position) -> Result<(), &'a str> {
        // Check if either given positions are out of the bounds of the chessboard.
        if self.layout.get(old_pos.y).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout[old_pos.y].get(old_pos.x).is_none() {
            return Err("Error: Origin square is out of bounds!");
        }
        if self.layout.get(new_pos.y).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }
        if self.layout[new_pos.y].get(new_pos.x).is_none() {
            return Err("Error: Destination square is out of bounds!");
        }

        let old_square = &self.layout[old_pos.y][old_pos.x];
        let new_square = &self.layout[new_pos.y][new_pos.x];
        
        // Check if player is trying to move an empty square.
        if let Square::Empty = old_square {
            return Err("Error: An empty square cannot be moved!");
        }
        
        let old_square_colour = old_square.get_colour().unwrap();

        // Check if player is trying to move opponent's pieces.
        if old_square_colour != &self.player {
            return Err("Error: You cannot move your opponent's pieces!");
        }

        // Check if player is trying to move one of their pieces onto another one of their pieces.
        if let Some(new_square_colour) = new_square.get_colour() {
            if old_square_colour == new_square_colour {
                return Err(
                    "Error: You cannot move your own piece onto another one of your pieces!",
                );
            }
        };

        // Valid piece move checks here...

        Ok(())
    }

    fn next_turn(&mut self) {
        self.player = match self.player {
            PieceColour::Black => PieceColour::White,
            PieceColour::White => PieceColour::Black,
        }
    }
}

/// Contains pre-made layouts that can be used when creating a custom chessboard.
pub struct Layouts;

impl Layouts {
    /// Returns the standard chessboard layout.
    pub fn standard() -> Vec<Vec<Square>> {
        vec![
            vec![
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Queen,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::King,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::Black,
                },
            ],
            vec![
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::Black,
                },
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
                Square::Empty,
            ],
            vec![
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Pawn,
                    piece_colour: PieceColour::White,
                },
            ],
            vec![
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Queen,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::King,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Queen,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::King,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Bishop,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Knight,
                    piece_colour: PieceColour::White,
                },
                Square::Piece {
                    piece_type: PieceType::Rook,
                    piece_colour: PieceColour::White,
                },
            ],
        ]
    }
}
