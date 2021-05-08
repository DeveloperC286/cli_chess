#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::io::{stdin, stdout, Write};

use console::Term;

mod board;
mod movement;
mod piece;
mod position;
mod practice;
mod utilities;

fn main() {
    pretty_env_logger::init();
    print_introduction_screen();
    repl();
}

fn print_introduction_screen() {
    match Term::stdout().clear_screen() {
        Ok(_) => {
            println!("{}", utilities::WIZARD);
            println!("Welcome my little Chess Wizard!");
        }
        Err(error) => {
            error!("{}", error);
        }
    }
}

fn repl() {
    loop {
        match read().trim() {
            "help" => {
                println!("exit - exit the application.");
                println!("practice - start a new practice game.");
            }
            "exit" => {
                std::process::exit(0);
            }
            "practice" => {
                practice::repl();
            }
            _ => {
                println!("Did not understand the command...");
            }
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    println!("What would you like to do [practice]?");
    print!(" > ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    buffer
}
