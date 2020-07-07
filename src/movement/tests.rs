use super::*;

use insta::assert_json_snapshot;
use rstest::rstest;

//1. e4  e5  2. Nf3  Nc6  3. Bc4 Bc5 4. c3 Qe7 5. O-O d6 6. d4 Bb6 7. a4 a6 8. a5 Ba7 9. h3 Nf6 10. dxe5 Nxe5 11. Nxe5 Qxe5 12. Nd2 Bxh3 13. gxh3 Qg3+ 14. Kh1 Qxh3+ 15. Kg1 Ng4 16. Nf3 Qg3+ 17. Kh1 Bxf2
#[rstest(
    movement,
    snapshot_name,
    case("e4", "von_scheve_vs_teichmann_game_1_berlin_1907_1w"),
    case("e5", "von_scheve_vs_teichmann_game_1_berlin_1907_1b"),
    case("Nf3", "von_scheve_vs_teichmann_game_1_berlin_1907_2w"),
    case("Nc6", "von_scheve_vs_teichmann_game_1_berlin_1907_2b"),
    case("Bc4", "von_scheve_vs_teichmann_game_1_berlin_1907_3w"),
    case("Bc5", "von_scheve_vs_teichmann_game_1_berlin_1907_3b"),
    case("c3", "von_scheve_vs_teichmann_game_1_berlin_1907_4w"),
    case("Qe7", "von_scheve_vs_teichmann_game_1_berlin_1907_4b")
)]
fn von_scheve_vs_teichmann_game_1_berlin_1907(movement: &str, snapshot_name: &str) {
    //when
    let movement = to_movement(movement);

    //then
    assert_json_snapshot!(snapshot_name, movement);
}
