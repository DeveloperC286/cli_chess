use std::collections::HashMap;

use crate::board::utilities::{get_positions_with_class, get_positions_with_colour};
use crate::model::movement::Movement;
use crate::model::piece::class::Class;
use crate::model::piece::colour::Colour;
use crate::model::piece::Piece;
use crate::model::position::file::File;
use crate::model::position::Position;
use crate::model::position::rank::Rank;

pub mod initial_board;
mod pawn;
pub mod utilities;

pub fn move_piece(
    turn: Colour,
    movement: Movement,
    piece_positions: &HashMap<Position, Piece>,
) -> Option<HashMap<Position, Piece>> {
    let current_colours_positions = get_positions_with_colour(
        piece_positions.keys().copied().collect(),
        piece_positions,
        turn,
    );
    let positions = pawn::get_pawns_that_can_make_move(
        get_positions_with_class(current_colours_positions, piece_positions, Class::Pawn),
        movement,
    );

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
                "'{}' pieces can make that move, be more specific.",
                positions.len()
            );
        }
    }

    None
}

pub fn get_board(piece_positions: &HashMap<Position, Piece>) -> Vec<String> {
    let mut board: Vec<String> = vec![];

    for rank in Rank::iter().rev() {
        let mut rank_representation = String::new();
        for file in File::iter() {
            match piece_positions.get(&Position::new(*file, *rank)) {
                Some(piece) => rank_representation.push_str(&format!(" {}", piece.get_character())),
                None => rank_representation.push_str(" -"),
            }
        }
        board.push(rank_representation);
    }

    board
}

pub fn print_board(board: Vec<String>, print_rank_and_file: bool) {
    for (rank_offset, rank) in board.iter().enumerate() {
        if print_rank_and_file {
            println!("{} {}", 8 - rank_offset, rank);
        } else {
            println!("{}", rank);
        }
    }

    if print_rank_and_file {
        println!();
        println!("   a b c d e f g h");
    }
}

#[cfg(test)]
mod tests;
