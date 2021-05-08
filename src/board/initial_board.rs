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
        let black_rank = Rank::_8;
        initial_board.insert(
            Position::new(File::A, black_rank),
            Piece::new(Class::Rook, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::B, black_rank),
            Piece::new(Class::Knight, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::C, black_rank),
            Piece::new(Class::Bishop, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::D, black_rank),
            Piece::new(Class::Queen, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::E, black_rank),
            Piece::new(Class::King, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::F, black_rank),
            Piece::new(Class::Bishop, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::G, black_rank),
            Piece::new(Class::Knight, Colour::Black),
        );
        initial_board.insert(
            Position::new(File::H, black_rank),
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
        let white_rank = Rank::_1;
        initial_board.insert(
            Position::new(File::A, white_rank),
            Piece::new(Class::Rook, Colour::White),
        );
        initial_board.insert(
            Position::new(File::B, white_rank),
            Piece::new(Class::Knight, Colour::White),
        );
        initial_board.insert(
            Position::new(File::C, white_rank),
            Piece::new(Class::Bishop, Colour::White),
        );
        initial_board.insert(
            Position::new(File::D, white_rank),
            Piece::new(Class::Queen, Colour::White),
        );
        initial_board.insert(
            Position::new(File::E, white_rank),
            Piece::new(Class::King, Colour::White),
        );
        initial_board.insert(
            Position::new(File::F, white_rank),
            Piece::new(Class::Bishop, Colour::White),
        );
        initial_board.insert(
            Position::new(File::G, white_rank),
            Piece::new(Class::Knight, Colour::White),
        );
        initial_board.insert(
            Position::new(File::H, white_rank),
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
