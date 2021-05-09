#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod mode;
mod model;
mod reporter;

fn main() {
    pretty_env_logger::init();
    crate::mode::menu::Menu::repl();
}
