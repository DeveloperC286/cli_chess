use insta::assert_debug_snapshot;
use rstest::rstest;

use super::*;

#[rstest(
    movement,
    snapshot_name,
    case("e4", "test_movement_case1"),
    case("ke4", "test_movement_case2"),
    case("A7", "test_movement_case3"),
    case("c2", "test_movement_case4"),
    case("Nd1", "test_movement_case5")
)]
pub fn test_movement(movement: &str, snapshot_name: &str) {
    // When
    let movement = Movement::from(movement);

    // Then
    assert_debug_snapshot!(snapshot_name, movement);
}

#[rstest(
    movement,
    snapshot_name,
    case("k4", "test_movement_invalid_file_case1"),
    case("N6", "test_movement_invalid_file_case2"),
    case("kN2", "test_movement_invalid_file_case2")
)]
pub fn test_movement_invalid_file(movement: &str, snapshot_name: &str) {
    // When
    let movement = Movement::from(movement);

    // Then
    assert_debug_snapshot!(snapshot_name, movement);
}

#[rstest(
    movement,
    snapshot_name,
    case("a0", "test_movement_invalid_rank_case1"),
    case("E9", "test_movement_invalid_rank_case2"),
    case("kC9", "test_movement_invalid_rank_case3")
)]
pub fn test_movement_invalid_rank(movement: &str, snapshot_name: &str) {
    // When
    let movement = Movement::from(movement);

    // Then
    assert_debug_snapshot!(snapshot_name, movement);
}

#[rstest(
    movement,
    snapshot_name,
    case("ca2", "test_movement_invalid_class_case1"),
    case("LE6", "test_movement_invalid_class_case2"),
    case("AC7", "test_movement_invalid_class_case3")
)]
pub fn test_movement_invalid_class(movement: &str, snapshot_name: &str) {
    // When
    let movement = Movement::from(movement);

    // Then
    assert_debug_snapshot!(snapshot_name, movement);
}

#[rstest(
    movement,
    snapshot_name,
    case("e44", "test_movement_invalid_length_case1"),
    case("ke74", "test_movement_invalid_length_case2"),
    case("PA77", "test_movement_invalid_length_case3"),
    case("c222", "test_movement_invalid_length_case4"),
    case("Nd12", "test_movement_invalid_length_case5")
)]
pub fn test_movement_invalid_length(movement: &str, snapshot_name: &str) {
    // When
    let movement = Movement::from(movement);

    // Then
    assert_debug_snapshot!(snapshot_name, movement);
}
