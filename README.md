[cargo-xtask]: https://github.com/matklad/cargo-xtask

[devx-cmd-docs-rs]: https://docs.rs/devx-cmd
[devx-cmd-docs-rs-badge]: https://docs.rs/devx-cmd/badge.svg
[devx-cmd-crates-io]: https://crates.io/crates/devx-cmd
[devx-cmd-crates-io-badge]: https://img.shields.io/crates/v/devx-cmd.svg?logo=rust

[devx-pre-commit-docs-rs]: https://docs.rs/devx-pre-commit
[devx-pre-commit-docs-rs-badge]: https://docs.rs/devx-pre-commit/badge.svg
[devx-pre-commit-crates-io]: https://crates.io/crates/devx-pre-commit
[devx-pre-commit-crates-io-badge]: https://img.shields.io/crates/v/devx-pre-commit.svg?logo=rust

<h1 align="center">
    <pre>devx</pre>
</h1>

<div align="center">
    <a alt="GitHub Actions" href="https://github.com/elastio/devx/actions">
        <img src="https://github.com/elastio/devx/workflows/ci/badge.svg"/>
    </a>
    <a alt="Master docs" href="https://elastio.github.io/devx/devx_cmd/index.html">
        <img src="https://img.shields.io/badge/docs-master-green.svg"/>
    </a>
</div>

Devx is a collection of utilities for writing your own dev scripts in Rust.
The project is inspired by and intended for seamless usage with [`cargo-xtask` idioms (you are highly encouraged to study them first)][cargo-xtask].

Most notably it provides convenient APIs for:
- Spawing and interacting with external processes [`devx-cmd`][devx-cmd-docs-rs]
- Creating git pre-commit hooks that enforce good practices [`devx-pre-commit`][devx-pre-commit-docs-rs]

## Crate map


Crate | docs.rs | crates.io
--|--|--
`devx-cmd` | [![][devx-cmd-docs-rs-badge]][devx-cmd-docs-rs] | [![][devx-cmd-crates-io-badge]][devx-cmd-crates-io]
`devx-pre-commit` | [![][devx-pre-commit-docs-rs-badge]][devx-pre-commit-docs-rs] | [![][devx-pre-commit-crates-io-badge]][devx-pre-commit-crates-io]


## Goals

All `devx` crates take a convenience-first approach.
The goal is to provide developer-friendly, and laconic APIs as well as the shortest possible compile times.

In particular, this implies:
- Batteries-included
- A vanishingly small amount of dependencies
- Using concrete types and a limited amount of generics
- Logging is included
- Opaque dynamic errors
- Less borrowed and non-thread-safe types

The non-goals are
- Performance and robust consistency
- Usage in highly non-standard projects. Instead, we expect `cargo` layout and sane project configurations. Ideally, `devx` and `cargo-xtask` might become a framework for enforcing good practices for Rust projects development.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license, shall be
dually licensed as above, without any additional terms or conditions.
</sub>
