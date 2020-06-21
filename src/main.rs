mod cli_handler;
mod imply;
pub mod interface;
pub mod utility;
extern crate chrono;
pub mod file;
// use termion::terminal_size;


fn main() {
    cli_handler::run();
}
