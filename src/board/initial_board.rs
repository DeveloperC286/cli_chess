use std::collections::HashMap;

use crate::model::piece::class::Class;
use crate::model::piece::colour::Colour;
use crate::model::piece::Piece;
use crate::model::position::file::File;
use crate::model::position::Position;
use crate::model::position::rank::Rank;

pub fn get_initial_board() -> HashMap<Position, Piece> {
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
    initial_board.insert(
        Position::new(File::A, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_7),
        Piece::new(Class::Pawn, Colour::Black),
    );

    //White
    initial_board.insert(
        Position::new(File::A, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_2),
        Piece::new(Class::Pawn, Colour::White),
    );
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

    initial_board
}
