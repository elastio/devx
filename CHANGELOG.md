[Unreleased]: https://github.com/elastio/devx/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/elastio/devx/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/elastio/devx/commits/v0.1.0

[`log`]: https://docs.rs/log

[#22]: https://github.com/elastio/devx/pull/22

# Changelog

All notable changes to `devx` crates are documented here. Crates' versions are
always bumped all at once.

## [Unreleased]

### Changed

#### devx-cmd

- Use [`log`] crate for logging
- **BREAKING** Replaced `Cmd::echo_cmd(bool)` -> `Cmd::log_cmd(impl Into<Option<log::Level>>)`
- **BREAKING** Replaced `Cmd::echo_err(bool)` -> `Cmd::log_err(impl Into<Option<log::Level>>)`

## [0.2.0] - 2020-08-22

### Changed

#### devx-cmd

- **BREAKING** Renamed `ChildProcess` -> `Child` ([#22])
- Stdout of the child process is always logged from the new line ([#22])

## [0.1.0] - 2020-08-17

- Initial release
