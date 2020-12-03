use super::*;

//1. e4  e5  2. Nf3  Nc6  3. Bc4 Bc5 4. c3 Qe7 5. O-O d6 6. d4 Bb6 7. a4 a6 8. a5 Ba7 9. h3 Nf6 10. dxe5 Nxe5 11. Nxe5 Qxe5 12. Nd2 Bxh3 13. gxh3 Qg3+ 14. Kh1 Qxh3+ 15. Kg1 Ng4 16. Nf3 Qg3+ 17. Kh1 Bxf2
#[test]
fn von_scheve_vs_teichmann_game_1_berlin_1907() {
    let mut piece_positions = crate::board::initial_board::get_initial_board();

    let mut movement = to_movement("e4").unwrap();
    piece_positions = move_piece(Colour::WHITE, movement, &piece_positions).unwrap();
    assert_json_snapshot!("1w", get_board(&piece_positions));

    movement = to_movement("e5").unwrap();
    piece_positions = move_piece(Colour::BLACK, movement, &piece_positions).unwrap();
    assert_json_snapshot!("1b", get_board(&piece_positions));

    movement = to_movement("Nf3").unwrap();
    piece_positions = move_piece(Colour::WHITE, movement, &piece_positions).unwrap();
    /*    assert_json_snapshot!(
        "2w",
        get_board(&piece_positions)
    );*/

    movement = to_movement("Nc6").unwrap();
    piece_positions = move_piece(Colour::BLACK, movement, &piece_positions).unwrap();
    /*    assert_json_snapshot!(
        "2b",
        get_board(&piece_positions)
    );*/
}
