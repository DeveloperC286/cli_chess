pub fn print_board(board: Vec<String>, print_rank_and_file: bool) {
    for (rank_offset, rank) in board.iter().enumerate() {
        if print_rank_and_file {
            println!("{} {}", 8 - rank_offset, rank);
        } else {
            println!("{}", rank);
        }
    }

    if print_rank_and_file {
        println!();
        println!("   a b c d e f g h");
    }
}
