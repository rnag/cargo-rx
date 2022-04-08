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
            // enable verbose output
            verbose: 1,
            // uncomment to build in `--release` mode
            // release: true,
            jobs: Some(2),
            profile: Some("release".to_owned()),
            features: vec!["feature-1".to_owned()],
            no_default_features: true,
            ignore_rust_version: true,
            color: Some("always".to_owned()),
            locked: true,
            offline: true,
            config: vec!["my_cfg='test'".to_owned()],
            ..Default::default()
        },
        ..Default::default()
    })
    .unwrap();
}
