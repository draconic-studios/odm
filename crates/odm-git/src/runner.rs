use std::ffi::{OsStr, OsString};
use std::io;
use std::process::{Command, ExitStatus, Output, Stdio};

/// Result of one process invocation.
#[derive(Debug, Clone)]
pub struct CommandOutput {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl CommandOutput {
    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }

    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }
}

/// Injectable process runner (real git or test double).
pub trait CommandRunner {
    fn output(
        &self,
        program: &OsStr,
        args: &[OsString],
    ) -> io::Result<CommandOutput>;

    /// Inherit stdio (for `project git` passthrough).
    fn status(&self, program: &OsStr, args: &[OsString]) -> io::Result<ExitStatus>;
}

/// Shells out via `std::process::Command`.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn output(&self, program: &OsStr, args: &[OsString]) -> io::Result<CommandOutput> {
        let out: Output = Command::new(program).args(args).output()?;
        Ok(CommandOutput {
            status: out.status,
            stdout: out.stdout,
            stderr: out.stderr,
        })
    }

    fn status(&self, program: &OsStr, args: &[OsString]) -> io::Result<ExitStatus> {
        Command::new(program)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
    }
}

