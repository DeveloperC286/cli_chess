use insta::assert_debug_snapshot;
use rstest::rstest;

use super::*;

#[rstest(
    position,
    snapshot_name,
    case("e4", "test_position_case1"),
    case("E5", "test_position_case2"),
    case("f3", "test_position_case3"),
    case("C6", "test_position_case4"),
    case("a7", "test_position_case5"),
    case("b1", "test_position_case6"),
    case("D7", "test_position_case7")
)]
pub fn test_position(position: &str, snapshot_name: &str) {
    // When
    let position = Position::from(position);

    // Then
    assert_debug_snapshot!(snapshot_name, position);
}

#[rstest(
    position,
    snapshot_name,
    case("l4", "test_position_invalid_file_case1"),
    case("p4", "test_position_invalid_file_case2"),
    case("z4", "test_position_invalid_file_case3")
)]
pub fn test_position_invalid_file(position: &str, snapshot_name: &str) {
    // When
    let position = Position::from(position);

    // Then
    assert_debug_snapshot!(snapshot_name, position);
}

#[rstest(
    position,
    snapshot_name,
    case("a0", "test_position_invalid_rank_case1"),
    case("c9", "test_position_invalid_rank_case2")
)]
pub fn test_position_invalid_rank(position: &str, snapshot_name: &str) {
    // When
    let position = Position::from(position);

    // Then
    assert_debug_snapshot!(snapshot_name, position);
}

#[rstest(
    position,
    snapshot_name,
    case("e11", "test_position_invalid_length_case1"),
    case("a1a1", "test_position_invalid_length_case2"),
    case("E99", "test_position_invalid_length_case3"),
    case("c18", "test_position_invalid_length_case4")
)]
pub fn test_position_invalid_length(position: &str, snapshot_name: &str) {
    // When
    let position = Position::from(position);

    // Then
    assert_debug_snapshot!(snapshot_name, position);
}
