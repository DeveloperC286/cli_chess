use std::io::{stdin, stdout, Write};

use crate::board::{get_board, move_piece, print_board};
use crate::board::initial_board::get_initial_board;
use crate::model::piece::colour::Colour;
use crate::movement::to_movement;

pub fn repl() {
    println!("New practice game...");
    let mut piece_positions = get_initial_board();
    let mut colours_turn = Colour::White;
    let mut turn = 0;
    let mut moves = String::new();
    let mut toggle_board = false;
    let mut print_rank_and_file = false;

    help();
    loop {
        let input = read().trim().to_string();
        match &*input {
            "help" => {
                help();
            }
            "exit" => {
                return;
            }
            "board" => {
                print_board(get_board(&piece_positions), print_rank_and_file);
            }
            "toggle-board" => {
                toggle_board = !toggle_board;
            }
            "toggle-rand-and-file" => {
                print_rank_and_file = !print_rank_and_file;
            }
            _ => match to_movement(&*input) {
                Some(movement) => {
                    if let Some(updated_piece_positions) =
                        move_piece(colours_turn, movement, &piece_positions)
                    {
                        //move piece
                        piece_positions = updated_piece_positions;

                        //update state for next turn
                        if colours_turn == Colour::White {
                            turn += 1;
                            moves.push_str(&format!("{}. ", turn));
                        }
                        colours_turn = next_turn(colours_turn);
                        moves.push_str(&format!("{} ", input));
                        println!("{}", moves);
                    }

                    if toggle_board {
                        println!();
                        print_board(get_board(&piece_positions), print_rank_and_file);
                    }
                }
                None => {
                    println!("'{}' is not a recognised command or movement.", input);
                }
            },
        }
    }
}

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

fn next_turn(colour: Colour) -> Colour {
    match colour {
        Colour::White => Colour::Black,
        Colour::Black => Colour::White,
    }
}

fn read() -> String {
    let mut buffer = String::new();

    print!(" > ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    buffer
}
