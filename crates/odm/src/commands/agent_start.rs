//! `odm agent start` — one-shot exec in a Project/wt cwd.

use odm_actions::{start_agent, StartOptions, StartResult, StdioMode};
use odm_core::OdmError;
use serde::Serialize;

use crate::ctx::Ctx;
use crate::present::{print_json, GlobalOut};

/// `odm agent start --json` envelope.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AgentStartDto {
    pub cwd: String,
    pub program: String,
    pub args: Vec<String>,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub enum StartOutcome {
    /// JSON mode: print DTO then exit with child code.
    Json(AgentStartDto),
    /// Inherit stdio: child already wrote; return raw exit.
    Inherit(i32),
}

pub fn agent_start_dto(
    program: impl Into<String>,
    args: &[String],
    result: &StartResult,
) -> AgentStartDto {
    AgentStartDto {
        cwd: result.cwd.display().to_string(),
        program: program.into(),
        args: args.to_vec(),
        exit_code: result.exit_code,
        stdout: result.stdout.clone().unwrap_or_default(),
        stderr: result.stderr.clone().unwrap_or_default(),
    }
}

/// Resolve project/program and one-shot exec via start lib.
pub fn start_cmd(
    ctx: &Ctx,
    program_args: &[String],
    json: bool,
) -> Result<StartOutcome, OdmError> {
    let project = ctx
        .project
        .as_deref()
        .filter(|p| !p.is_empty())
        .ok_or_else(|| OdmError::usage("project is required"))?;
    let (program, args) = match program_args {
        [] => return Err(OdmError::usage("program is required")),
        [program, rest @ ..] => {
            if program.is_empty() {
                return Err(OdmError::usage("program is required"));
            }
            (program.as_str(), rest)
        }
    };
    let stdio = if json {
        StdioMode::Capture
    } else {
        StdioMode::Inherit
    };
    let result = start_agent(
        &ctx.ws,
        StartOptions {
            project,
            wt: ctx.wt.as_deref(),
            program,
            args,
            stdio,
        },
    )?;
    if json {
        Ok(StartOutcome::Json(agent_start_dto(program, args, &result)))
    } else {
        Ok(StartOutcome::Inherit(result.exit_code))
    }
}

/// Finish helper for start outcomes (special-case inherit / JSON exit).
pub fn finish_start(_out: &GlobalOut, outcome: StartOutcome) -> Result<i32, OdmError> {
    match outcome {
        StartOutcome::Json(dto) => {
            print_json(&dto)?;
            Ok(dto.exit_code)
        }
        StartOutcome::Inherit(code) => Ok(code),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn agent_start_dto_shape() {
        let result = StartResult {
            exit_code: 0,
            cwd: PathBuf::from("/tmp/ws/projects/alpha"),
            stdout: Some("out\n".into()),
            stderr: Some("err\n".into()),
        };
        let args = vec!["a".into(), "b".into()];
        let dto = agent_start_dto("echo", &args, &result);
        let v = serde_json::to_value(&dto).unwrap();
        assert_eq!(v["cwd"], "/tmp/ws/projects/alpha");
        assert_eq!(v["program"], "echo");
        assert_eq!(v["args"], serde_json::json!(["a", "b"]));
        assert_eq!(v["exitCode"], 0);
        assert_eq!(v["stdout"], "out\n");
        assert_eq!(v["stderr"], "err\n");
    }

    #[test]
    fn agent_start_dto_empty_streams() {
        let result = StartResult {
            exit_code: 1,
            cwd: PathBuf::from("/p"),
            stdout: None,
            stderr: None,
        };
        let dto = agent_start_dto("false", &[], &result);
        assert_eq!(dto.stdout, "");
        assert_eq!(dto.stderr, "");
        assert_eq!(dto.exit_code, 1);
        assert!(dto.args.is_empty());
    }
}
