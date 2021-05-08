use console::Term;

pub struct Menu {}

impl Menu {
    pub fn repl() {
        fn print_introduction_screen() {
            match Term::stdout().clear_screen() {
                Ok(_) => {
                    println!("{}", crate::mode::utilities::WIZARD);
                    println!("Welcome my little Chess Wizard!");
                }
                Err(error) => {
                    error!("{}", error);
                }
            }
        }

        fn help() {
            println!();
            println!("help - print this message.");
            println!("exit - exit the application.");
            println!("practice - start a new practice game, where you play each colour.");
            println!();
        }

        print_introduction_screen();
        help();

        loop {
            match crate::mode::utilities::read_stdin().trim() {
                "help" => {
                    help();
                }
                "exit" => {
                    break;
                }
                "practice" => {
                    crate::mode::practice::Practice::repl();
                }
                _ => {
                    println!("Did not understand the command...");
                }
            }
        }
    }
}
