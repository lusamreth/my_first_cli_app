mod imply;
mod interface;
extern crate chrono;
// use clap::{Arg, App,SubCommand};
fn main() {
    imply::mainrunner();
    interface::run();
    // println!("P{:#?}",test_clap);
}