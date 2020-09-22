# Changelog

All notable changes to `devx` crates are documented here. Crates' versions are
always bumped all at once.

## [Unreleased]

- Soon to come...

## [0.3.1] - 2020-09-22

### Added

#### devx-cmd

- Added `Cmd::env` method to set environment variables for the processes ([#35])

## [0.3.0] - 2020-08-29

### Changed

#### devx-cmd

- Use [`log`] crate for logging ([#30])
- **BREAKING:** Replaced `Cmd::echo_cmd(bool)` -> `Cmd::log_cmd(impl Into<Option<log::Level>>)` ([#30])
- **BREAKING:** Replaced `Cmd::echo_err(bool)` -> `Cmd::log_err(impl Into<Option<log::Level>>)` ([#30])

#### devx-pre-commit

- Replaced `std::fs` with [`fs-err`] crate internally ([#31])

## [0.2.0] - 2020-08-22

### Changed

#### devx-cmd

- **BREAKING:** Renamed `ChildProcess` -> `Child` ([#22])
- Stdout of the child process is always logged from the new line ([#22])

## [0.1.0] - 2020-08-17

- Initial release

[Unreleased]: https://github.com/elastio/devx/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/elastio/devx/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/elastio/devx/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/elastio/devx/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/elastio/devx/commits/v0.1.0

[`log`]: https://docs.rs/log
[`fs-err`]: https://docs.rs/fs-err

[#22]: https://github.com/elastio/devx/pull/22
[#30]: https://github.com/elastio/devx/pull/30
[#31]: https://github.com/elastio/devx/pull/31
[#35]: https://github.com/elastio/devx/pull/35
