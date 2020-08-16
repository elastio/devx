[cargo-xtask]: https://github.com/matklad/cargo-xtask

# Devx

Devx is a collection of utilities for writing your own dev scripts in Rust.
The project is inspired by and intended for seamless usage with [`cargo-xtask` idioms (you are highly encouraged to study them first)][cargo-xtask].

Most notably it provides convenient APIs for:
- Spawing and interacting with external processes [`devx-cmd`](./devx-cmd/README.md)
- Creating git pre-commit hooks that enforce good practices [`devx-pre-commit`](./devx-pre-commit/README.md)

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
