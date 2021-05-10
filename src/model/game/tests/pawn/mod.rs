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