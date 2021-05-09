use super::*;

#[test]
fn test_game_1() {
    // Theodor von Scheve vs Richard Teichmann, Berlin 1907
    // 1. e4  e5  2. Nf3  Nc6  3. Bc4 Bc5 4. c3 Qe7 5. O-O d6 6. d4 Bb6
    // 7. a4 a6 8. a5 Ba7 9. h3 Nf6 10. dxe5 Nxe5 11. Nxe5 Qxe5 12. Nd2 Bxh3
    // 13. gxh3 Qg3+ 14. Kh1 Qxh3+ 15. Kg1 Ng4 16. Nf3 Qg3+ 17. Kh1 Bxf2

    let mut game = crate::model::game::Game::new();

    let movement = Movement::from("e4").unwrap();
    game.move_piece(movement).unwrap();
    assert_debug_snapshot!("1w", game.get_board_representation());

    let movement = Movement::from("e5").unwrap();
    game.move_piece(movement).unwrap();
    assert_debug_snapshot!("1b", game.get_board_representation());
}
