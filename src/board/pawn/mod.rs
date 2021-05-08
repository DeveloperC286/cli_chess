use crate::board::utilities::get_positions_with_file;
use crate::model::position::Position;
use crate::movement::Movement;

pub fn get_pawns_that_can_make_move(positions: Vec<Position>, movement: Movement) -> Vec<Position> {
    get_positions_with_file(positions, movement.destination.file)
    //    //TODO pawn blocked logic
    //     //TODO pawn stacked logic
    //     //TODO pawn take logic
    //     //TODO pawn one move max after first move logic
}
