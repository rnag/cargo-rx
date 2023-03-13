//! This example is a custom use-case where we have Cargo
//! (binary) crates within the `examples/` folder.
//!
//! # Note
//!
//! This is a use case that not even `cargo run --example`
//! currently supports -- in fact, this is not even
//! detected as an example otherwise!
//!
//! # Sample Projects With This Setup
//!
//! * [axum](https://github.com/tokio-rs/axum)
//! * [tract](https://github.com/sonos/tract)
//!
fn main() {
    println!("Hello, world!");
}
