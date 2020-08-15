use anyhow::Result;
use devx_pre_commit::PreCommitContext;

pub fn run_hook() -> Result<()> {
    let ctx = PreCommitContext::from_git_diff(crate::project_root_dir())?;
    ctx.rustfmt()?;
    ctx.stage_new_changes()?;
    Ok(())
}

pub fn install_hook() -> Result<()> {
    devx_pre_commit::install_self_as_hook(crate::project_root_dir())?;
    Ok(())
}
