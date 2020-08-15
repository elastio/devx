[`rustfmt`]: https://github.com/rust-lang/rustfmt

# devx-pre-commit

`devx-pre-commit` provides utilities for creating git pre-commit hooks.

In particular, there are convenient APIs for
- Efficiently running [`rustfmt`] on crates with staged rust source files
- Installing the current binary to `.git/hooks/pre-commit`
