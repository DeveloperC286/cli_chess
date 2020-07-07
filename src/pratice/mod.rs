use std::io::{stdin, stdout, Write};

use crate::board::{get_board, get_initial_board, move_piece, print_board};
use crate::movement::to_movement;
use crate::piece::Colour;

pub fn repl() {
    println!("New pratice game...");
    let mut piece_positions = get_initial_board();
    let mut colours_turn = Colour::WHITE;
    let mut turn = 0;
    let mut moves = String::new();

    loop {
        let input = read().trim().to_string();
        match &*input {
            "help" => {
                println!("exit - return to the main menu.");
                println!("board - print the chess board.");
            }
            "exit" => {
                return;
            }
            "board" => {
                print_board(get_board(&piece_positions));
            }
            _ => match to_movement(&*input) {
                Some(movement) => match move_piece(colours_turn, movement, &piece_positions) {
                    Some(updated_piece_positions) => {
                        //move piece
                        piece_positions = updated_piece_positions;

                        //update state for next turn
                        if colours_turn == Colour::WHITE {
                            turn += 1;
                            moves.push_str(&format!("{}. ", turn));
                        }
                        colours_turn = next_turn(colours_turn);
                        moves.push_str(&format!("{} ", input));
                        println!("{}", moves);
                    }
                    None => {}
                },
                None => {
                    println!("'{}' is not a recongised command or movement.", input);
                }
            },
        }
    }
}

fn next_turn(colour: Colour) -> Colour {
    match colour {
        Colour::WHITE => Colour::BLACK,
        Colour::BLACK => Colour::WHITE,
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

    return buffer;
}
