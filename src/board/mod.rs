use crate::movement::Movement;
use crate::piece::{get_character, Class, Colour, Piece};
use crate::position::file::File;
use crate::position::rank::Rank;
use crate::position::Position;
use std::collections::HashMap;

fn get_positions_with_colour(
    positions: Vec<Position>,
    piece_positions: &HashMap<Position, Piece>,
    colour: Colour,
) -> Vec<Position> {
    return positions
        .iter()
        .filter(|position| piece_positions.get(position).unwrap().colour == colour)
        .map(|x| *x)
        .collect();
}

fn get_positions_with_file(positions: Vec<Position>, file: File) -> Vec<Position> {
    return positions
        .iter()
        .filter(|position| position.file == file)
        .map(|x| *x)
        .collect();
}

fn get_positions_with_class(
    positions: Vec<Position>,
    piece_positions: &HashMap<Position, Piece>,
    class: Class,
) -> Vec<Position> {
    return positions
        .iter()
        .filter(|position| piece_positions.get(position).unwrap().class == class)
        .map(|x| *x)
        .collect();
}

pub fn move_piece(
    turn: Colour,
    movement: Movement,
    piece_positions: &HashMap<Position, Piece>,
) -> Option<HashMap<Position, Piece>> {
    let mut positions: Vec<Position> = piece_positions.keys().map(|x| *x).collect();
    positions = get_positions_with_colour(positions, piece_positions, turn);
    positions = get_positions_with_file(positions, movement.destination.file);
    positions = get_positions_with_class(positions, piece_positions, Class::PAWN);
    //TODO pawn blocked logic
    //TODO pawn stacked logic
    //TODO pawn take logic
    //TODO pawn one move max after first move logic

    match positions.len() {
        1 => {
            let mut updated_piece_positions = piece_positions.clone();
            let old_position = positions.first().unwrap();
            //move pawn
            updated_piece_positions.insert(
                movement.destination,
                *updated_piece_positions.get(old_position).unwrap(),
            );
            updated_piece_positions.remove(old_position);
            return Some(updated_piece_positions);
        }
        0 => {
            println!("No pieces can make that move.");
        }
        _ => {
            println!(
                "'{}' pieces can make that move, be more specfic.",
                positions.len()
            );
        }
    }

    return None;
}

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

pub fn get_board(piece_positions: &HashMap<Position, Piece>) -> Vec<String> {
    let mut board: Vec<String> = vec![];

    for rank in Rank::iterator().rev() {
        let mut rank_representation = String::new();
        for file in File::iterator() {
            match piece_positions.get(&Position::new(*file, *rank)) {
                Some(piece) => rank_representation.push_str(&format!(" {}", get_character(*piece))),
                None => rank_representation.push_str(" -"),
            }
        }
        board.push(rank_representation);
    }

    return board;
}

pub fn print_board(board: Vec<String>) {
    for rank in board {
        println!("{}", rank);
    }
}

#[cfg(test)]
mod tests;
