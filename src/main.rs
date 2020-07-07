extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use console::Term;

mod board;
mod movement;
mod piece;
mod position;
mod pratice;
mod utilities;

use std::io::{stdin, stdout, Write};

fn main() {
    pretty_env_logger::init();
    print_introduction_screen();
    repl();
}

fn print_introduction_screen() {
    let term = Term::stdout();
    term.clear_screen();
    println!("{}", utilities::WIZARD);
    println!("Welcome my little Chess Wizard!");
}

fn repl() {
    loop {
        match read().trim() {
            "help" => {
                println!("exit - exit the application.");
                println!("pratice - start a new pratice game.");
            }
            "exit" => {
                std::process::exit(0);
            }
            "pratice" => {
                pratice::repl();
            }
            _ => {
                println!("Did not understand the command...");
            }
        }
    }
}

fn read() -> String {
    let mut buffer = String::new();

    println!("What would you like to do [pratice]?");
    print!(" > ");
    let _ = stdout().flush();

    match stdin().read_line(&mut buffer) {
        Ok(_n) => {}
        Err(error) => error!("Error reading user input: {}", error),
    }

    return buffer;
}
