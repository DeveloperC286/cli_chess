use std::collections::HashMap;

use crate::model::movement::Movement;
use crate::model::piece::class::Class;
use crate::model::piece::colour::Colour;
use crate::model::piece::Piece;
use crate::model::position::file::File;
use crate::model::position::rank::Rank;
use crate::model::position::Position;

mod initial_piece_positions;

pub struct Game {
    turn: Colour,
    piece_positions: HashMap<Position, Piece>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Colour::White,
            piece_positions: crate::model::game::initial_piece_positions::INITIAL_PIECE_POSITIONS
                .clone(),
        }
    }

    pub fn move_piece(&mut self, movement: Movement) -> Result<(), ()> {
        let potential_pieces_to_move: HashMap<Position, Piece> = self
            .piece_positions
            .clone()
            .into_iter()
            .filter(|(_, piece)| piece.colour == self.turn)
            .collect();

        let possible_pawns_to_move: HashMap<Position, Piece> = potential_pieces_to_move
            .into_iter()
            .filter(|(_, piece)| piece.class == Class::Pawn)
            .filter(|(position, _)| position.file == movement.destination.file)
            .filter(|(position, _)| position.rank.difference(movement.destination.rank) <= 2)
            .collect();

        let mut possible_pieces_to_move: HashMap<Position, Piece> = HashMap::new();
        possible_pieces_to_move.extend(possible_pawns_to_move);

        match possible_pieces_to_move.len() {
            1 => {
                // Remove from old position.
                let (old_position, piece) = possible_pieces_to_move.into_iter().next().unwrap();
                self.piece_positions.remove(&old_position);

                // Move to new position.
                self.piece_positions.insert(movement.destination, piece);

                match self.turn {
                    Colour::White => self.turn = Colour::Black,
                    Colour::Black => self.turn = Colour::White,
                }

                return Ok(());
            }
            0 => {
                println!("No pieces can make that move.");
            }
            _ => {
                println!("Multiple pieces can make that move, be more specific.");
            }
        }

        Err(())
    }

    pub fn get_board_representation(&self) -> Vec<String> {
        Rank::iter()
            .rev()
            .map(|rank| {
                let mut rank_representation = String::new();

                for file in File::iter() {
                    match self.piece_positions.get(&Position::new(*file, *rank)) {
                        Some(piece) => {
                            rank_representation.push_str(&format!(" {}", piece.get_character()))
                        }
                        None => rank_representation.push_str(" -"),
                    }
                }

                rank_representation
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
