use std::collections::HashMap;

use crate::model::piece::class::Class;
use crate::model::piece::colour::Colour;
use crate::model::piece::Piece;
use crate::model::position::file::File;
use crate::model::position::rank::Rank;
use crate::model::position::Position;

lazy_static! {
    pub static ref INITIAL_BOARD: HashMap<Position, Piece> = {
        let mut initial_board = HashMap::new();
        // Black
        initial_board.insert(
            Position::new(File::A, Rank::_8),
            Piece::new(Class::Rook, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::B, Rank::_8),
            Piece::new(Class::Knight, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::C, Rank::_8),
            Piece::new(Class::Bishop, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::D, Rank::_8),
            Piece::new(Class::Queen, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::E, Rank::_8),
            Piece::new(Class::King, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::F, Rank::_8),
            Piece::new(Class::Bishop, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::G, Rank::_8),
            Piece::new(Class::Knight, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::H, Rank::_8),
            Piece::new(Class::Rook, Colour::Black),
        );

        // Black pawns
        for file in File::iter() {
            initial_board.insert(
                Position::new(*file, Rank::_7),
                Piece::new(Class::Pawn, Colour::Black),
            );
        }

        //White
        initial_board.insert(
            Position::new(File::A, Rank::_1),
            Piece::new(Class::Rook, Colour::White),
        );
        initial_board.insert(
            Position::new(File::B, Rank::_1),
            Piece::new(Class::Knight, Colour::White),
        );
        initial_board.insert(
            Position::new(File::C, Rank::_1),
            Piece::new(Class::Bishop, Colour::White),
        );
        initial_board.insert(
            Position::new(File::D, Rank::_1),
            Piece::new(Class::Queen, Colour::White),
        );
        initial_board.insert(
            Position::new(File::E, Rank::_1),
            Piece::new(Class::King, Colour::White),
        );
        initial_board.insert(
            Position::new(File::F, Rank::_1),
            Piece::new(Class::Bishop, Colour::White),
        );
        initial_board.insert(
            Position::new(File::G, Rank::_1),
            Piece::new(Class::Knight, Colour::White),
        );
        initial_board.insert(
            Position::new(File::H, Rank::_1),
            Piece::new(Class::Rook, Colour::White),
        );

        // White pawns
        for file in File::iter() {
            initial_board.insert(
                Position::new(*file, Rank::_2),
                Piece::new(Class::Pawn, Colour::White),
            );
        }

        initial_board
    };
}
