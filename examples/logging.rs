//! Simple example which demonstrates the ideal approach
//! to printing out log output and messages via the
//! ubiquitous `logging` crate.
//!
#[macro_use]
extern crate log;

fn main() {
    sensible_env_logger::init!();

    trace!("Hello, world!");
    info!("This is an {} message!", "INFO");
}
