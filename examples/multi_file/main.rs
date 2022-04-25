//! This script demonstrates a [Multi-file example].
//!
//! Note that Cargo supports adding sub-directories
//! inside of the *examples/* folder, and looks for
//! a `main.rs` inside of them to build the example.
//!
//! [Multi-file example]: https://doc.bccnsoft.com/docs/rust-1.36.0-docs-html/edition-guide/rust-2018/cargo-and-crates-io/multi-file-examples.html
//!
mod utils;

fn main() {
    utils::print_hello();
    println!("this is a multi-file example!");
}
