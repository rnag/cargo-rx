//! Runs an sample `example` script for demo purposes.
//!
//! This script runs the `hello_world` example with
//! a *name* along with a *number* of times to greet
//! an individual.
//!
use cargo_rx::{process_input, Args};

fn main() {
    let args = ["--name", "John Smitty", "--count", "2"];

    process_input(Args {
        name: Some("hello_world".into()),
        args: Vec::from(args.map(String::from)),
        input_args: false,
        cargo: cargo_options::CommonOptions {
            verbose: 1,
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
