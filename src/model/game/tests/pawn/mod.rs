use super::*;

#[rstest(movement, case("e5"), case("e6"), case("e7"), case("e8"))]
fn test_white_initial_one_or_two_rank_only(movement: &str) {
    // Given
    let mut game = crate::model::game::Game::new();

    // When/Then
    let movement = Movement::from(movement).unwrap();
    game.move_piece(movement).unwrap_err();
}

#[rstest(movement, case("e4"), case("e3"), case("e2"), case("e1"))]
fn test_black_initial_one_or_two_rank_only(movement: &str) {
    // Given
    let mut game = crate::model::game::Game::new();
    game.move_piece(Movement::from("a3").unwrap()).unwrap();

    // When/Then
    let movement = Movement::from(movement).unwrap();
    game.move_piece(movement).unwrap_err();
}

#[rstest(file, case("a"), case("b"), case("c"), case("d"), case("e"), case("f"))]
fn test_white_only_one_after_initial(file: &str) {
    // Given
    let mut game = crate::model::game::Game::new();
    // First white move
    game.move_piece(Movement::from(&*format!("{}3", file)).unwrap())
        .unwrap();
    // First black move
    game.move_piece(Movement::from("a6").unwrap()).unwrap();

    // When/Then
    for rank in 5..9 {
        // Second white move can't move more than one
        game.move_piece(Movement::from(&*format!("{}{}", file, rank)).unwrap())
            .unwrap_err();
    }

    game.move_piece(Movement::from(&*format!("{}4", file)).unwrap())
        .unwrap();
}

#[rstest(file, case("a"), case("b"), case("c"), case("d"), case("e"), case("f"))]
fn test_black_only_one_after_initial(file: &str) {
    // Given
    let mut game = crate::model::game::Game::new();
    // First white move
    game.move_piece(Movement::from("a3").unwrap()).unwrap();
    // First black move
    game.move_piece(Movement::from(&*format!("{}6", file)).unwrap())
        .unwrap();
    // Second white move
    game.move_piece(Movement::from("a4").unwrap()).unwrap();

    // When/Then
    for rank in 1..5 {
        // Second black move can't move more than one
        game.move_piece(Movement::from(&*format!("{}{}", file, rank)).unwrap())
            .unwrap_err();
    }

    game.move_piece(Movement::from(&*format!("{}5", file)).unwrap())
        .unwrap();
}

//TODO taking.
