use std::collections::HashMap;

use crate::piece::{Class, Colour, Piece};
use crate::position::file::File;
use crate::position::rank::Rank;
use crate::position::Position;

pub fn get_initial_board() -> HashMap<Position, Piece> {
    let mut initial_board = HashMap::new();

    // Black
    initial_board.insert(
        Position::new(File::A, Rank::_8),
        Piece::new(Class::Rook, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_8),
        Piece::new(Class::Knight, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_8),
        Piece::new(Class::Bishop, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_8),
        Piece::new(Class::Queen, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_8),
        Piece::new(Class::King, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_8),
        Piece::new(Class::Bishop, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_8),
        Piece::new(Class::Knight, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_8),
        Piece::new(Class::Rook, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::A, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_7),
        Piece::new(Class::Pawn, Colour::BLACK),
    );

    //White
    initial_board.insert(
        Position::new(File::A, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_2),
        Piece::new(Class::Pawn, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::A, Rank::_1),
        Piece::new(Class::Rook, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_1),
        Piece::new(Class::Knight, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_1),
        Piece::new(Class::Bishop, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_1),
        Piece::new(Class::Queen, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_1),
        Piece::new(Class::King, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_1),
        Piece::new(Class::Bishop, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_1),
        Piece::new(Class::Knight, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_1),
        Piece::new(Class::Rook, Colour::WHITE),
    );

    initial_board
}
