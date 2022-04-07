//! Simple program to greet a person
//!
//! # Arguments
//!
//! `name` - The name of the person to greet
//! `count` - The number of times to greet
//!
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    println!("Hello world!");
    println!();

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
