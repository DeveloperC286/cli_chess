use crate::model::game::Game;
use crate::model::movement::Movement;

pub struct Practice {}

impl Practice {
    pub fn repl() {
        fn help() {
            println!();
            println!("help - print this message.");
            println!("exit - return to the main menu.");
            println!("board - print the chess board.");
            println!("toggle-board - print the chess board after ever move.");
            println!("toggle-rand-and-file - print the rank and file with the chess board.");
            println!("<chess move> - move a chess piece.");
            println!();
        }

        println!("New practice game...");
        let mut game = Game::new();
        let mut toggle_board = false;
        let mut print_rank_and_file = false;

        help();
        loop {
            let input = crate::mode::utilities::read_stdin().trim().to_string();
            match &*input {
                "help" => {
                    help();
                }
                "exit" => {
                    return;
                }
                "board" => {
                    crate::reporter::print_board(
                        game.get_board_representation(),
                        print_rank_and_file,
                    );
                }
                "toggle-board" => {
                    toggle_board = !toggle_board;
                }
                "toggle-rand-and-file" => {
                    print_rank_and_file = !print_rank_and_file;
                }
                _ => match Movement::from(&input) {
                    Some(movement) => {
                        if let Ok(()) = game.move_piece(movement) {
                            if toggle_board {
                                println!();
                                crate::reporter::print_board(
                                    game.get_board_representation(),
                                    print_rank_and_file,
                                );
                            }
                        }
                    }
                    None => {
                        println!("'{}' is not a recognised command or movement.", input);
                    }
                },
            }
        }
    }
}
