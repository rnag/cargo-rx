# cargo-rx

[<img alt="github" src="https://img.shields.io/badge/github-source-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rnag/cargo-rx)
[<img alt="crates.io" src="https://img.shields.io/crates/v/cargo-rx.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/cargo-rx)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/cargo-rx/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="20">](https://docs.rs/cargo-rx)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/rnag/cargo-rx/build.yml?branch=main&style=for-the-badge" height="20">](https://github.com/rnag/cargo-rx/actions/workflows/build.yml?query=branch%3Amain)

**`cargo-rx` is a simple fuzzy finder and *R*unner for *Ex*amples in a [Cargo] project.**

<a href="https://asciinema.org/a/483363">
  <img src="https://asciinema.org/a/483363.svg" alt="rx demo" height="50%" width="70%" />
</a>

[Cargo]: http://doc.crates.io/

This crate provides a single executable: `rx`.
Basically anywhere you would use `cargo run --example` in a
Rust project, try `rx` instead.

**If this project has helped you, please consider making a [donation](https://www.buymeacoffee.com/ritviknag).**

<!-- TODO
## Getting started
-->

## Table of Contents

* [Installation](#installation)
  * [From source](#from-source)
  * [From binaries](#from-binaries)
  * [Windows](#windows)
* [Features](#features)
* [Contributing](#contributing)
* [Buy me a coffee](#buy-me-a-coffee)
* [License](#license)
* [Authors](#authors)

## Installation

### From source

Ensure that you have a fairly recent version of [rust/cargo] installed. Then, run:

```shell
$ cargo install cargo-rx
```

*Compiler support: requires rustc 1.58+*

[rust/cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html

### From binaries

Check out the [Releases page] for pre-built binaries compatible
with various architectures.

[Releases page]: https://github.com/rnag/cargo-rx/releases

### Windows

Note that on a *Windows* environment, you will also need to have [fzf installed]
and available in your *$PATH* variable.

An easy way to install fzf is via [Chocolatey]:

```console
choco install fzf
```

There is currently a [feature request] open on `skim` which proposes adding
support for Windows, but this has not been currently implemented yet --
thus, the `fzf` tool serves as a stand-in alternative for now.

[fzf installed]: https://github.com/junegunn/fzf#windows
[Chocolatey]: https://chocolatey.org/packages/fzf
[feature request]: https://github.com/lotabout/skim/issues/293

## Features

* Fuzzy finder, which leverages [skim] to sort and search for *examples* in a Cargo project -- when called with just `rx`.
* Pass arguments after `--` to the selected example.
* Automatically [enables required-features] when running an example.
* Play back of most recently run example via the `--replay` option.
* Automatically enables `+nightly` toolchain when passed in *unstable options* to `cargo run`, such as `--unit-graph`.

[skim]: https://github.com/lotabout/skim
[enables required-features]: https://github.com/rust-lang/cargo/issues/4663

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/rnag/cargo-rx/issues

## Buy me a coffee

Liked some of my work? Buy me a coffee (or more likely a beer)

<a href="https://www.buymeacoffee.com/ritviknag" target="_blank"><img src="https://bmc-cdn.nyc3.digitaloceanspaces.com/BMC-button-images/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;"></a>

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`cargo-rx` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* `cargo-rx` is brought to you by [Ritvik Nag](https://github.com/rnag).
