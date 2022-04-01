# cargo-rx

[<img alt="github" src="https://img.shields.io/badge/github-rnag/cargo--rx-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/rnag/cargo-rx)
[<img alt="crates.io" src="https://img.shields.io/crates/v/cargo-rx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/cargo-rx)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/cargo-rx/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/cargo-rx)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rnag/cargo-rx/build/main?style=for-the-badge" height="22">](https://github.com/rnag/cargo-rx/actions?query=branch%3Amain)

**`cargo-rx` is a simple, modern *R*unner for *Ex*amples in a [Cargo] project.**

[Cargo]: http://doc.crates.io/

---

this crate provides a single executable: `rx`.
Basically anywhere you would use `cargo run --example` in a
Rust project, run `rx` instead.

<!-- TODO
## Getting started
-->

# Installation

Ensure that you have a fairly recent version of rust/cargo installed. Then, run:

```shell
$ cargo install cargo-rx
```

*Compiler support: requires rustc 1.58+*

(Please check [`cargo`'s documentation] to learn how `cargo install` works and how to set up your system so it finds binaries installed by `cargo`.)

[`cargo`'s documentation]: https://doc.rust-lang.org/cargo/

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/cargo-rx/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`cargo-rx` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Ritvik Nag](https://github.com/rnag)
