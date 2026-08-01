use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::ExitStatus;

use crate::error::{trim_output, GitError};
use crate::runner::{CommandRunner, ProcessRunner};

/// Shell-out git façade for ODM multi-git lifecycle.
///
/// Paths must be absolute. Library ops use `git -C <path>` and capture stdio.
/// [`Git::run`] inherits stdio for `odm project git` passthrough.
#[derive(Debug, Clone)]
pub struct Git<R: CommandRunner = ProcessRunner> {
    runner: R,
    program: OsString,
}

impl Default for Git<ProcessRunner> {
    fn default() -> Self {
        Self::new()
    }
}

impl Git<ProcessRunner> {
    pub fn new() -> Self {
        Self::with_runner(ProcessRunner)
    }
}

impl<R: CommandRunner> Git<R> {
    pub fn with_runner(runner: R) -> Self {
        Self {
            runner,
            program: OsString::from("git"),
        }
    }

    pub fn with_program(mut self, program: impl Into<OsString>) -> Self {
        self.program = program.into();
        self
    }

    pub fn is_repo(&self, path: &Path) -> Result<bool, GitError> {
        require_absolute(path)?;
        if !path.exists() {
            return Ok(false);
        }
        let out = self.capture(
            "is_repo",
            None,
            &["-C".into(), path.into(), "rev-parse".into(), "--is-inside-work-tree".into()],
        )?;
        if !out.status.success() {
            return Ok(false);
        }
        Ok(trim_output(out.stdout_str()) == "true")
    }

    pub fn init(&self, path: &Path) -> Result<(), GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "init",
            Some(path),
            &["init".into(), path.into()],
        )?;
        if !out.status.success() {
            return Err(GitError::failed(
                "init",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(())
    }

    pub fn clone(
        &self,
        url: &str,
        path: &Path,
        branch: Option<&str>,
    ) -> Result<(), GitError> {
        require_absolute(path)?;
        let mut args: Vec<OsString> = vec!["clone".into()];
        if let Some(b) = branch {
            args.push("-b".into());
            args.push(b.into());
        }
        args.push(url.into());
        args.push(path.into());
        let out = self.capture("clone", Some(path), &args)?;
        if !out.status.success() {
            return Err(GitError::failed(
                "clone",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(())
    }

    pub fn fetch(&self, path: &Path) -> Result<(), GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "fetch",
            Some(path),
            &["-C".into(), path.into(), "fetch".into()],
        )?;
        if !out.status.success() {
            return Err(GitError::failed(
                "fetch",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(())
    }

    pub fn head_sha(&self, path: &Path) -> Result<String, GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "head_sha",
            Some(path),
            &["-C".into(), path.into(), "rev-parse".into(), "HEAD".into()],
        )?;
        if !out.status.success() {
            return Err(GitError::failed(
                "head_sha",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        let sha = trim_output(out.stdout_str()).to_ascii_lowercase();
        if !is_full_sha(&sha) {
            return Err(GitError::Parse {
                operation: "head_sha",
                stdout: sha,
                detail: "expected 40-char hex SHA".into(),
            });
        }
        Ok(sha)
    }

    pub fn is_clean(&self, path: &Path) -> Result<bool, GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "is_clean",
            Some(path),
            &[
                "-C".into(),
                path.into(),
                "status".into(),
                "--porcelain=v1".into(),
                "-uall".into(),
            ],
        )?;
        if !out.status.success() {
            return Err(GitError::failed(
                "is_clean",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(out.stdout_str().trim().is_empty())
    }

    pub fn origin_url(&self, path: &Path) -> Result<String, GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "origin_url",
            Some(path),
            &[
                "-C".into(),
                path.into(),
                "remote".into(),
                "get-url".into(),
                "origin".into(),
            ],
        )?;
        if !out.status.success() {
            let stderr = trim_output(out.stderr_str());
            if looks_like_missing_origin(&stderr) {
                return Err(GitError::OriginMissing {
                    path: path.to_path_buf(),
                });
            }
            return Err(GitError::failed(
                "origin_url",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(trim_output(out.stdout_str()))
    }

    pub fn checkout_detached(&self, path: &Path, rev: &str) -> Result<(), GitError> {
        require_absolute(path)?;
        let out = self.capture(
            "checkout_detached",
            Some(path),
            &[
                "-C".into(),
                path.into(),
                "checkout".into(),
                "--detach".into(),
                rev.into(),
            ],
        )?;
        if !out.status.success() {
            return Err(GitError::failed(
                "checkout_detached",
                Some(path.to_path_buf()),
                out.status,
                out.stderr_str(),
                out.stdout_str(),
            ));
        }
        Ok(())
    }

    /// Passthrough: `git -C <path> <args…>`. Inherits stdio. Empty args → [`GitError::EmptyArgs`].
    pub fn run(&self, path: &Path, args: &[impl AsRef<OsStr>]) -> Result<ExitStatus, GitError> {
        require_absolute(path)?;
        if args.is_empty() {
            return Err(GitError::EmptyArgs);
        }
        let mut full: Vec<OsString> = vec!["-C".into(), path.into()];
        full.extend(args.iter().map(|a| a.as_ref().to_os_string()));
        self.runner
            .status(&self.program, &full)
            .map_err(map_io)
    }

    fn capture(
        &self,
        operation: &'static str,
        path: Option<&Path>,
        args: &[OsString],
    ) -> Result<crate::runner::CommandOutput, GitError> {
        self.runner
            .output(&self.program, args)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    GitError::GitNotFound(e)
                } else {
                    GitError::Failed {
                        operation,
                        path: path.map(Path::to_path_buf),
                        code: None,
                        stderr: e.to_string(),
                    }
                }
            })
    }
}

fn require_absolute(path: &Path) -> Result<(), GitError> {
    if path.is_absolute() {
        Ok(())
    } else {
        Err(GitError::NotAbsolute(path.to_path_buf()))
    }
}

fn map_io(e: std::io::Error) -> GitError {
    if e.kind() == std::io::ErrorKind::NotFound {
        GitError::GitNotFound(e)
    } else {
        GitError::Failed {
            operation: "run",
            path: None,
            code: None,
            stderr: e.to_string(),
        }
    }
}

fn is_full_sha(s: &str) -> bool {
    s.len() == 40 && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn looks_like_missing_origin(stderr: &str) -> bool {
    let lower = stderr.to_ascii_lowercase();
    lower.contains("no such remote") || lower.contains("not a remote")
}

