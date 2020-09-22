//! This a dev cli, see `HELP` strings for details.

use anyhow::Result;
use std::env;
use xtask::pre_commit;

const HELP: &str = "\
xtask
This binary defines auxiliary ad-hoc scripts that serve as a replacement for
nasty enormous bash scripts. This is a dev cli, and it is intended to be used
only by the developers and not for production distribution.
See https://github.com/matklad/cargo-xtask/ for more info.

USAGE:
    xtask <SUBCOMMAND>

SUBCOMMANDS:
    install-pre-commit-hook    Install git pre-commit hook that formats code with `cargo fmt`";

fn main() -> Result<()> {
    simple_logger::SimpleLogger::new().init().unwrap();

    if let Some(true) = env::args().next().map(|it| it.contains("pre-commit")) {
        return pre_commit::run_hook();
    }

    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()?.unwrap_or_default().as_str() {
        "install-pre-commit-hook" => {
            args.finish()?;
            pre_commit::install_hook()?;
            eprintln!("Git pre-commit hook is successfully installed. Have a nice day :D");
        }
        _ => eprintln!("{}", HELP),
    }
    Ok(())
}
