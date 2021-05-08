#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;

mod board;
mod mode;
mod model;

fn main() {
    pretty_env_logger::init();
    crate::mode::menu::Menu::repl();
}
