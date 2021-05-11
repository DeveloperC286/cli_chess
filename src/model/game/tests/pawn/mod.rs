use super::*;

fn new_movement(file: File, rank: Rank) -> Movement {
    Movement {
        class: None,
        destination: Position { file, rank },
    }
}

const WHITE_RANKS: [Rank; 4] = [Rank::_5, Rank::_6, Rank::_7, Rank::_8];
const WHITE_POSSIBLE_RANKS: [Rank; 2] = [Rank::_3, Rank::_4];
const BLACK_RANKS: [Rank; 4] = [Rank::_1, Rank::_2, Rank::_3, Rank::_4];
const BLACK_POSSIBLE_RANKS: [Rank; 2] = [Rank::_5, Rank::_6];

#[test]
fn test_white_initial_pawn_move_can_move_one_or_two_ranks() {
    for file in File::iter() {
        for rank in WHITE_POSSIBLE_RANKS.iter() {
            // Given
            let mut game = crate::model::game::Game::new();

            // When/Then
            assert_eq!(Ok(()), game.move_piece(new_movement(*file, *rank)));
        }
    }
}

#[test]
fn test_white_initial_pawn_move_cant_move_more_than_two_ranks() {
    // Given
    let mut game = crate::model::game::Game::new();

    // When/Then
    for file in File::iter() {
        for rank in WHITE_RANKS.iter() {
            assert_eq!(
                Err(MovementError::NoPieceCanMakeMove),
                game.move_piece(new_movement(*file, *rank))
            );
        }
    }
}

#[test]
fn test_black_initial_pawn_move_can_move_one_or_two_ranks() {
    for file in File::iter() {
        for rank in BLACK_POSSIBLE_RANKS.iter() {
            // Given
            let mut game = crate::model::game::Game::new();
            game.move_piece(Movement::from("a3").unwrap()).unwrap();

            // When/Then
            assert_eq!(Ok(()), game.move_piece(new_movement(*file, *rank)));
        }
    }
}

#[test]
fn test_black_initial_pawn_move_cant_move_more_than_two_ranks() {
    // Given
    let mut game = crate::model::game::Game::new();
    game.move_piece(Movement::from("a3").unwrap()).unwrap();

    // When/Then
    for file in File::iter() {
        for rank in BLACK_RANKS.iter() {
            assert_eq!(
                Err(MovementError::NoPieceCanMakeMove),
                game.move_piece(new_movement(*file, *rank))
            );
        }
    }
}

#[test]
fn test_white_only_one_after_initial() {
    for file in File::iter() {
        // Given
        let mut game = crate::model::game::Game::new();

        // First white move
        game.move_piece(new_movement(*file, Rank::_3)).unwrap();
        // First black move
        game.move_piece(Movement::from("a6").unwrap()).unwrap();

        // When/Then
        for rank in WHITE_RANKS.iter() {
            // Second white move can't move more than one
            assert_eq!(
                Err(MovementError::NoPieceCanMakeMove),
                game.move_piece(new_movement(*file, *rank))
            );
        }
    }
}

#[test]
fn test_black_only_one_after_initial() {
    for file in File::iter() {
        // Given
        let mut game = crate::model::game::Game::new();
        // First white move
        game.move_piece(Movement::from("a3").unwrap()).unwrap();
        // First black move
        game.move_piece(new_movement(*file, Rank::_6)).unwrap();
        // Second white move
        game.move_piece(Movement::from("a4").unwrap()).unwrap();

        // When/Then
        for rank in BLACK_RANKS.iter() {
            // Second black move can't move more than one
            assert_eq!(
                Err(MovementError::NoPieceCanMakeMove),
                game.move_piece(new_movement(*file, *rank))
            );
        }
    }
}

//TODO blocked.
//Blocked on init 2 move
//Blocked on init 1 move
//BLocked general case in middle

//TODO taking.
