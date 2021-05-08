use std::collections::HashMap;

use crate::model::class::Class;
use crate::model::colour::Colour;
use crate::model::piece::Piece;
use crate::position::file::File;
use crate::position::Position;

pub fn get_positions_with_colour(
    positions: Vec<Position>,
    piece_positions: &HashMap<Position, Piece>,
    colour: Colour,
) -> Vec<Position> {
    positions
        .iter()
        .filter(|position| piece_positions.get(position).unwrap().colour == colour)
        .copied()
        .collect()
}

pub fn get_positions_with_class(
    positions: Vec<Position>,
    piece_positions: &HashMap<Position, Piece>,
    class: Class,
) -> Vec<Position> {
    positions
        .iter()
        .filter(|position| piece_positions.get(position).unwrap().class == class)
        .copied()
        .collect()
}

pub(crate) fn get_positions_with_file(positions: Vec<Position>, file: File) -> Vec<Position> {
    positions
        .iter()
        .filter(|position| position.file == file)
        .copied()
        .collect()
}
