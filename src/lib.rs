#![doc(html_root_url = "https://docs.rs/cargo-rx/0.3.0")]
#![warn(rust_2018_idioms, missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]

//! [![github]](https://github.com/rnag/cargo-rx)&ensp;[![crates-io]](https://crates.io/crates/cargo-rx)&ensp;[![docs-rs]](https://docs.rs/cargo-rx)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! > **`cargo-rx` is a simple, modern *R*unner for *Ex*amples in a Cargo project.**
//!
//! A fuzzy finder tool which extends [Cargo] to allow you to easily search and run examples
//! from the command line.
//!
//! This crate provides a single executable: `rx`.
//!
//! [Cargo]: http://doc.crates.io/
//!
//!
//! <br>
//!
//! ## Demo
//!
//! [![rx demo](https://asciinema.org/a/483363.svg)](https://asciinema.org/a/483363)
//!
//! Basically anywhere you would use `cargo run --example` in a Rust project, try `rx` instead.
//!
//! ## Install
//!
//! Ensure that you have a fairly recent version of [rust/cargo] installed. Then, run:
//!
//! [rust/cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html
//! ```shell
//! $ cargo install cargo-rx
//! ```
//! *Compiler support: requires rustc 1.58+*
//!
//! ### Windows
//!
//! Note that on a *Windows* environment, you will also need to have [fzf installed]
//! and available in your *$PATH* variable.
//!
//! An easy way to install `fzf` is via [Chocolatey]:
//!
//! ```console
//! choco install fzf
//! ```
//!
//! There is currently a [feature request] open on `skim` which proposes adding
//! support for Windows, but this has not been currently implemented yet --
//! thus, the `fzf` tool serves as a stand-in alternative for now.
//!
//! [fzf installed]: https://github.com/junegunn/fzf#windows
//! [Chocolatey]: https://chocolatey.org/packages/fzf
//! [feature request]: https://github.com/lotabout/skim/issues/293
//!
//! ## Usage
//!
//! Once in a Cargo project with an `examples/` folder, run:
//!
//! ```console
//! $ rx
//! ```
//!
//! If you want to run a specific example, you can alternatively do that as well:
//!
//! ```console
//! $ rx my_example -- --my-arg "argument to pass in to example"
//! ```
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/rnag/cargo-rx/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/cargo-rx
//! [`README.md`]: https://github.com/rnag/cargo-rx
//!

mod cache;
mod constants;
mod models;
// noinspection SpellCheckingInspection
mod osstringext;
mod pathbufext;
mod run_ext;
mod run_impl;
mod types;

use cache::*;
pub use constants::*;
pub use models::*;
pub use osstringext::*;
pub use pathbufext::*;
pub use run_ext::*;
pub(crate) use run_impl::*;
pub use types::*;

/// Processes an input to *select or run* an **example** in a [Cargo] project.
///
/// [Cargo]: http://doc.crates.io/
pub fn process_input(args: Args) -> Result<()> {
    #[cfg(target_family = "windows")]
    patch_colored_for_windows();

    let p = Paths::resolve()?;

    let files = p.example_files()?;

    process_input_inner(files, &p, args)
}

/// This is a **patch** so that the `colored` output works as expected
/// when the `rx` binary is installed with `cargo install` in a
/// *Windows* environment.
///
/// See [the linked issue] for more details.
///
/// [the linked issue]: https://github.com/mackwic/colored/issues/76
#[cfg(target_family = "windows")]
#[inline]
fn patch_colored_for_windows() {
    colored::control::set_virtual_terminal(true).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
