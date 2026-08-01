mod cli;
mod output;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use odm_core::{
    discover_root, init_workspace, load_workspace, OdmError, InitOptions,
};

use cli::{Cli, Commands, PinCmd, ProjectCmd};
use output::{print_error, print_json, GlobalOut};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let out = GlobalOut {
        json: cli.json,
    };

    match run(cli, &out) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let code = print_error(&out, &e);
            ExitCode::from(code as u8)
        }
    }
}

fn run(cli: Cli, out: &GlobalOut) -> Result<(), OdmError> {
    match cli.command {
        Commands::Init {
            path,
            no_git,
            interactive,
        } => {
            if interactive {
                return Err(OdmError::not_implemented("init --interactive"));
            }
            let target = path.unwrap_or_else(|| PathBuf::from("."));
            // --root does not apply to init target; init uses path arg / cwd
            let res = init_workspace(InitOptions {
                path: target,
                no_git,
                name: None,
            })?;
            if out.json {
                print_json(&serde_json::json!({
                    "root": res.root,
                    "git": res.git,
                }))?;
            } else {
                println!(
                    "Initialized Workspace at {} (git: {})",
                    res.root.display(),
                    res.git
                );
            }
            Ok(())
        }
        Commands::Sync { .. } => Err(OdmError::not_implemented("sync")),
        Commands::Pin { cmd } => match cmd {
            PinCmd::Apply { .. } => Err(OdmError::not_implemented("pin apply")),
            PinCmd::Status { .. } => Err(OdmError::not_implemented("pin status")),
        },
        Commands::Status => Err(OdmError::not_implemented("status")),
        Commands::Doctor { .. } => Err(OdmError::not_implemented("doctor")),
        Commands::Project { cmd } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            match cmd {
                ProjectCmd::List => {
                    if out.json {
                        let list: Vec<_> = ws
                            .config
                            .projects
                            .iter()
                            .map(|(name, e)| {
                                serde_json::json!({
                                    "name": name,
                                    "path": e.path,
                                    "url": e.url,
                                    "branch": e.branch,
                                    "type": e.type_,
                                })
                            })
                            .collect();
                        print_json(&list)?;
                    } else if ws.config.projects.is_empty() {
                        println!("(no projects)");
                    } else {
                        for (name, e) in &ws.config.projects {
                            let managed = if e.url.is_some() { "managed" } else { "path" };
                            println!("{name}\t{}\t{managed}", e.path);
                        }
                    }
                    Ok(())
                }
                ProjectCmd::Add { .. } => Err(OdmError::not_implemented("project add")),
                ProjectCmd::Rm { .. } => Err(OdmError::not_implemented("project rm")),
                ProjectCmd::Info { name } => {
                    let entry = ws.config.projects.get(&name).ok_or_else(|| {
                        OdmError::usage(format!("unknown project '{name}'"))
                    })?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "name": name,
                            "path": entry.path,
                            "url": entry.url,
                            "branch": entry.branch,
                            "type": entry.type_,
                        }))?;
                    } else {
                        println!("name: {name}");
                        println!("path: {}", entry.path);
                        if let Some(u) = &entry.url {
                            println!("url: {u}");
                        }
                        if let Some(b) = &entry.branch {
                            println!("branch: {b}");
                        }
                        if let Some(t) = &entry.type_ {
                            println!("type: {t}");
                        }
                    }
                    Ok(())
                }
                ProjectCmd::Git { .. } => Err(OdmError::not_implemented("project git")),
            }
        }
    }
}
