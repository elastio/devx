//! `devx-pre-commit` provides utilities for creating git pre-commit hooks.
//!
//! In particular, there are convenient APIs for
//! - Efficiently running [`rustfmt`] on crates with staged rust source files
//! - Installing the current binary to `.git/hooks/pre-commit`
//!
//! This crate is meant to be used only in dev environment, preferably with
//! [`cargo-xtask`] setup. By having something like the code bellow in
//! `xtask` binary crate you will be able to run the following command to install
//! the git pre-commit hook and never bother running `cargo fmt` manually again:
//!
//! ```bash
//! cargo xtask install-pre-commit-hook
//! ```
//!
//! > ℹ️ Note: This assumes there is an alias in `.cargo/config`:
//! > ```toml
//! > [alias]
//! > xtask = "run --package xtask --bin xtask --"
//! > ```
//!
//! Example dev cli:
//! ```no_run
//! use devx_pre_commit::{PreCommitContext, locate_project_root};
//! use anyhow::Result;
//! use std::{ffi::OsStr, path::PathBuf};
//!
//! fn run_hook() -> Result<()> {
//!     let mut ctx = PreCommitContext::from_git_diff(locate_project_root()?)?;
//!
//!     // Optionally filter out the files you don't want to format
//!     ctx.retain_staged_files(|path| {
//!         path.components().all(|it| it.as_os_str() != OsStr::new("generated"))
//!     });
//!
//!     // Run `cargo fmt` against the crates with staged rust source files
//!     ctx.rustfmt()?;
//!
//!     // Stage all the changes potenitally introduced by rustfmt
//!     // It is super-important to call this method at the end of the hook
//!     ctx.stage_new_changes()?;
//!     Ok(())
//! }
//!
//! fn main() -> Result<()> {
//!     if let Some(true) = std::env::args().next().map(|it| it.contains("pre-commit")) {
//!         return run_hook();
//!     }
//!     match std::env::args().nth(1).expect("No args").as_str() {
//!         "install-pre-commit-hook" => {
//!             devx_pre_commit::install_self_as_hook(&locate_project_root()?)?;
//!         }
//!         _ => {
//!             eprintln!("Hi, this is a dev cli, here are the available commands...");
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! [`cargo-xtask`]: https://github.com/matklad/cargo-xtask
//! [`rustfmt`]: https://github.com/rust-lang/rustfmt
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
// Makes rustc abort compilation if there are any unsafe blocks in the crate.
// Presence of this annotation is picked up by tools such as cargo-geiger
// and lets them ensure that there is indeed no unsafe code as opposed to
// something they couldn't detect (e.g. unsafe added via macro expansion, etc).
#![forbid(unsafe_code)]

use fs_err as fs;
use std::{
    collections::HashSet,
    env::{self, consts},
    ffi::OsStr,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::Result;
use devx_cmd::{cmd, run};

/// Represents the API entrypoint of the git pre-commit hook.
/// It carries the list of the staged files and the project root path.
/// Note that staged file paths are all relative to the project root path.
pub struct PreCommitContext {
    staged_files: Vec<PathBuf>,
    project_root: PathBuf,
}

impl PreCommitContext {
    /// Creates the git pre-commit context acquiring the staged files via running
    /// `git diff` in `project_root`.
    /// The `project_root` is expected to contain the `.git` dir
    /// (see [`locate_project_root`] function for more on that).
    ///
    /// The staged files are stored in [`PreCommitContext`] as paths relative
    /// to `project_root`.
    pub fn from_git_diff(project_root: impl Into<PathBuf>) -> Result<Self> {
        let project_root = project_root.into();
        let diff = cmd!(
            "git",
            "diff",
            "--diff-filter",
            "MAR",
            "--name-only",
            "--cached"
        )
        .current_dir(&project_root)
        .read()?;

        Ok(Self {
            staged_files: diff.lines().map(PathBuf::from).collect(),
            project_root,
        })
    }

    /// Returns an iterator over all the files staged for the commit.
    pub fn staged_files(&self) -> impl Iterator<Item = &Path> {
        self.staged_files.iter().map(PathBuf::as_path)
    }

    /// Accepts a function predicate that accepts a relative path to the staged
    /// file and returns `false` if the given file should be removed from this
    /// [`PreCommitContext`]
    pub fn retain_staged_files(&mut self, mut f: impl FnMut(&Path) -> bool) {
        self.staged_files.retain(|it| f(it));
    }

    /// Returns the names of the crates that contain [`Self::staged_rust_files()`].
    ///
    /// Warning: this heuristically looks for `Cargo.toml` files and
    /// searches for `name = "` substring in them to get the crate name
    /// (i.e. it doesn't really parse them properly, but this works 99% of the
    /// time and lets us save on a full-fledged toml parser dependency).
    /// This heuristic may be relaxed in the future, and it shouldn't be considered a
    /// breaking change.
    pub fn touched_crates(&self) -> HashSet<String> {
        self.staged_rust_files()
            .filter_map(|rust_file_path| {
                rust_file_path.ancestors().find_map(|candidate| {
                    let cargo_toml = self.project_root.join(candidate).join("Cargo.toml");
                    let cargo_toml = fs::read_to_string(&cargo_toml).ok()?;

                    Self::parse_crate_name(&cargo_toml)
                })
            })
            .collect()
    }

    /// Returns an iterator over all staged files with `.rs` extension.
    pub fn staged_rust_files(&self) -> impl Iterator<Item = &Path> {
        self.staged_files
            .iter()
            .filter(|path| path.extension() == Some(OsStr::new("rs")))
            .map(PathBuf::as_path)
    }

    fn parse_crate_name(cargo_toml: &str) -> Option<String> {
        // FIXME: do some more robust toml parsing here:
        let name_prefix = "\nname = \"";
        let name = cargo_toml.find(name_prefix)? + name_prefix.len();
        let len = cargo_toml[name..]
            .find('"')
            .expect("Invalid toml, couldn't find closing double quote");
        Some(cargo_toml[name..name + len].to_owned())
    }

    /// Runs `cargo fmt` against the [`Self::touched_crates()`]
    pub fn rustfmt(&self) -> Result<()> {
        let touched_crates = self.touched_crates();
        if touched_crates.is_empty() {
            return Ok(());
        }

        cmd!(std::env::var("CARGO")
            .as_ref()
            .map(Deref::deref)
            .unwrap_or("cargo"))
        .arg("fmt")
        .arg("--package")
        .args(touched_crates)
        .run()?;

        Ok(())
    }

    /// Pushes the changes introduced to staged files in the working tree
    /// to the git index. It is important to call this function once you've
    /// modified some staged files (e.g. via [`Self::rustfmt()`])
    pub fn stage_new_changes(&self) -> Result<()> {
        run!("git", "update-index", "--again")?;
        Ok(())
    }
}

/// Copies the [`std::env::current_exe()`] file to `${project_root}/.git/hooks/pre-commit`
/// That's all you need to register a git pre-commit hook.
///
/// It will silently overwrite the existing git pre-commit hook.
pub fn install_self_as_hook(project_root: impl AsRef<Path>) -> Result<()> {
    let hook_path = project_root
        .as_ref()
        .join(".git")
        .join("hooks")
        .join("pre-commit")
        .with_extension(consts::EXE_EXTENSION);

    let me = env::current_exe()?;
    fs::copy(me, hook_path)?;

    Ok(())
}

/// Searches for a project root dir, which is a directory that contains
/// a `.git` dir as its direct child (it should also be the root of
/// the project's `Rust` crate or [cargo workspace][cargo-workspace]).
///
/// It uses the following steps:
/// 1. Use the value of [`$GIT_DIR`][git-dir] env variable if it is present.
/// (This variable is set by git when it invokes current process as a hook).
/// 2. Fallback to the output of [`git rev-parse --show-toplevel`][git-rev-parse].
///
/// [git-dir]: https://stackoverflow.com/a/37927943/9259330
/// [git-rev-parse]: https://git-scm.com/docs/git-rev-parse#Documentation/git-rev-parse.txt---show-toplevel
/// [cargo-workspace]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
pub fn locate_project_root() -> Result<PathBuf> {
    Ok(env::var("GIT_DIR").map(Into::into).or_else(|_| {
        cmd!("git", "rev-parse", "--show-toplevel")
            .read()
            .map(|it| it.trim_end().into())
    })?)
}
