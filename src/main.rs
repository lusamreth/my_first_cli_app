mod cli_handler;
mod imply;
pub mod interface;
pub mod utility;
extern crate chrono;
pub mod file;
fn main() {

    imply::mainrunner();
    cli_handler::run();
}
