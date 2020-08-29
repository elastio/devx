use crate::{Child, Cmd};
use std::fmt;

/// Shortcut for `Result<T, devx_cmd::Error>`
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Opaque error which happened during command execution.
#[derive(Debug)]
pub struct Error(String);

impl std::error::Error for Error {}

pub(crate) trait Context<T> {
    fn cmd_context(self, cmd: &Cmd) -> Result<T>;
    fn proc_context(self, proc: &Child) -> Result<T>;
}
impl<T, E: fmt::Display> Context<T> for Result<T, E> {
    fn cmd_context(self, cmd: &Cmd) -> Result<T> {
        self.map_err(|err| Error::cmd(cmd, &err))
    }
    fn proc_context(self, proc: &Child) -> Result<T> {
        self.map_err(|err| Error::proc(proc, &err))
    }
}

impl Error {
    fn new(msg: String, log_level: Option<log::Level>) -> Self {
        let me = Self(msg);
        if let Some(level) = log_level {
            log::log!(level, "[ERROR] {}", me.0);
        }
        me
    }

    fn cmd(cmd: &Cmd, message: &dyn fmt::Display) -> Self {
        let msg = format!("{}\nCommand: {}", message, cmd);
        Self::new(msg, cmd.0.log_err)
    }
    pub(crate) fn proc(proc: &Child, message: &dyn fmt::Display) -> Self {
        let msg = format!("{}\nProcess: {}", message, proc);
        Self::new(msg, proc.cmd.0.log_err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
