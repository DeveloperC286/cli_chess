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
        Piece::new(Class::ROOK, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_8),
        Piece::new(Class::KNIGHT, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_8),
        Piece::new(Class::BISHOP, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_8),
        Piece::new(Class::QUEEN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_8),
        Piece::new(Class::KING, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_8),
        Piece::new(Class::BISHOP, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_8),
        Piece::new(Class::KNIGHT, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_8),
        Piece::new(Class::ROOK, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::A, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_7),
        Piece::new(Class::PAWN, Colour::BLACK),
    );

    //White
    initial_board.insert(
        Position::new(File::A, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_2),
        Piece::new(Class::PAWN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::A, Rank::_1),
        Piece::new(Class::ROOK, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::B, Rank::_1),
        Piece::new(Class::KNIGHT, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::C, Rank::_1),
        Piece::new(Class::BISHOP, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::D, Rank::_1),
        Piece::new(Class::QUEEN, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::E, Rank::_1),
        Piece::new(Class::KING, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::F, Rank::_1),
        Piece::new(Class::BISHOP, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::G, Rank::_1),
        Piece::new(Class::KNIGHT, Colour::WHITE),
    );
    initial_board.insert(
        Position::new(File::H, Rank::_1),
        Piece::new(Class::ROOK, Colour::WHITE),
    );

    initial_board
}
