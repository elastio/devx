pub mod pre_commit;

use std::path::{Path, PathBuf};
use once_cell::sync::OnceCell;

fn project_root_dir() -> &'static Path {
    static DIR: OnceCell<PathBuf> = OnceCell::new();
    DIR.get_or_init(|| devx_pre_commit::locate_project_root().unwrap())
}
