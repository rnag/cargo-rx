# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.

## [Unreleased]
<!--
### Features
- Added a new struct `MyStruct` with the following methods:
  - `my_method()`
  - `other_method()`
-->

## v0.3.0 (2023-03-13)

### Features
- Add support for [Cargo crates with binaries] within the `examples/` folder,
  each containing their own `Cargo.toml` file. 🎉
  - This calls `cargo run --manifest-path <file>` internally, passing `--bin` in the case of multiple [binary targets].

[Cargo crates with binaries]: https://github.com/rnag/cargo-rx/issues/19
[binary targets]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries

## v0.2.0 (2022-04-25)

### Breaking Changes
- Rename short option for `--replay` to `-R`, since `-r` is now a shorthand for `--release`.

### Bug Fixes
- Patch `colored` output for Windows; the quickfix is implemented as suggested from [here](https://docs.rs/colored/1.9.3/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html).  🙌
- Display all example files supported by `cargo`, such as multi-file examples and ones with custom file paths. 🎉
- Examples displayed in the fuzzy finder are now properly sorted, A->Z as expected. 🖐️
- The terminal output of the command being run, i.e. `cargo run --example <selected example> [..args]`, now correctly displays arguments in quotes such as `--arg "Hello \"world\"!"`, which more closely matches the format of the command being run. 👍

### Features
- Add support for options to `cargo run --example`, such as `--release` and `--features`. 🎉
  - Automatically enables `+nightly` toolchain as required, when passed in *unstable options* to `cargo run` such as `--unit-graph`.
- Rewire `-p|--prompt-args` to `-i|--input-args`, but retained existing option for backwards-compatibility reasons.
- Update `Cargo.toml` dependencies.
  - Add `cargo-options` and `path-absolutize`
  - Update to use `home` instead of `dirs`, as overall it appears to be more stable

## v0.1.1 (2022-04-09)

- Update docs
- Update GH actions to publish binaries on the [Releases page]

[Releases page]: https://github.com/rnag/cargo-rx/releases

## v0.1.0 (2022-04-01)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/cargo-rx
