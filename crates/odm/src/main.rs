mod cli;
mod output;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use odm_core::{
    abs_checkout, build_status, discover_root, format_doctor_human, format_status_human,
    init_workspace, load_workspace, pin_apply, pin_status, project_add, project_git, project_rm,
    run_doctor, sync_managed, InitOptions, MaterializeOutcome, OdmError, ProjectEntry,
};
use odm_git::Git;

use cli::{Cli, Commands, PinCmd, ProjectCmd};
use output::{print_error, print_json, GlobalOut};

fn main() -> ExitCode {
    let cli = Cli::parse();
    let out = GlobalOut { json: cli.json };

    match run(cli, &out) {
        Ok(0) => ExitCode::SUCCESS,
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            let code = print_error(&out, &e);
            ExitCode::from(code as u8)
        }
    }
}

fn run(cli: Cli, out: &GlobalOut) -> Result<i32, OdmError> {
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
            Ok(0)
        }
        Commands::Sync { names } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let git = Git::new();
            let results = sync_managed(&git, &ws.root, &ws.config, &names)?;
            if out.json {
                print_json(&serde_json::json!({
                    "ok": true,
                    "results": results.iter().map(|r| {
                        serde_json::json!({
                            "name": r.name,
                            "materialized": match r.materialized {
                                MaterializeOutcome::Cloned => "cloned",
                                MaterializeOutcome::AlreadyPresent => "already_present",
                            },
                            "fetched": r.fetched,
                            "head": r.head,
                        })
                    }).collect::<Vec<_>>(),
                }))?;
            } else if results.is_empty() {
                println!("(no managed entries)");
            } else {
                for r in &results {
                    let mat = match r.materialized {
                        MaterializeOutcome::Cloned => "cloned",
                        MaterializeOutcome::AlreadyPresent => "present",
                    };
                    println!("{}\t{}\tfetched", r.name, mat);
                }
            }
            Ok(0)
        }
        Commands::Pin { cmd } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let git = Git::new();
            match cmd {
                PinCmd::Apply { names, force } => {
                    let results = pin_apply(&git, &ws.root, &ws.config, &names, force)?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "results": results.iter().map(|r| {
                                serde_json::json!({
                                    "name": r.name,
                                    "status": r.status,
                                    "rev": r.rev,
                                })
                            }).collect::<Vec<_>>(),
                        }))?;
                    } else {
                        for r in &results {
                            println!("{}\t{}\t{}", r.name, r.status, r.rev.as_deref().unwrap_or("-"));
                        }
                        if results.is_empty() {
                            println!("(nothing to apply)");
                        } else {
                            println!("applied");
                        }
                    }
                    Ok(0)
                }
                PinCmd::Status { names } => {
                    let report = pin_status(&git, &ws.root, &ws.config, &names)?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "pin_file": report.pin_file,
                            "present": report.present,
                            "entries": report.entries.iter().map(|e| {
                                serde_json::json!({
                                    "name": e.name,
                                    "pin_rev": e.pin_rev,
                                    "head": e.head,
                                    "state": e.state,
                                })
                            }).collect::<Vec<_>>(),
                        }))?;
                    } else if report.entries.is_empty() {
                        println!("pin file present: {}", report.present);
                        println!("(no managed entries)");
                    } else {
                        for e in &report.entries {
                            println!(
                                "{}\t{}\t{}",
                                e.name,
                                e.state,
                                e.pin_rev.as_deref().unwrap_or("-")
                            );
                        }
                    }
                    Ok(0)
                }
            }
        }
        Commands::Status => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let snap = build_status(&ws)?;
            if out.json {
                print_json(&snap)?;
            } else {
                print!("{}", format_status_human(&snap));
            }
            Ok(0)
        }
        Commands::Doctor { fix } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let report = run_doctor(&ws, fix)?;
            if out.json {
                print_json(&report)?;
            } else {
                print!("{}", format_doctor_human(&report));
            }
            if report.ok {
                Ok(0)
            } else {
                Ok(3)
            }
        }
        Commands::Project { cmd } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let mut ws = load_workspace(&root)?;
            let git = Git::new();
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
                        print_json(&serde_json::json!({ "projects": list }))?;
                    } else if ws.config.projects.is_empty() {
                        println!("(no projects)");
                    } else {
                        for (name, e) in &ws.config.projects {
                            let managed = if e.url.is_some() { "managed" } else { "path" };
                            println!("{name}\t{}\t{managed}", e.path);
                        }
                    }
                    Ok(0)
                }
                ProjectCmd::Add {
                    name,
                    path,
                    url,
                    branch,
                    type_,
                    no_clone,
                } => {
                    let rel = path_to_rel(&path)?;
                    let entry = ProjectEntry {
                        path: rel,
                        url,
                        branch,
                        type_,
                    };
                    let outcome =
                        project_add(&git, &ws.root, &mut ws.config, &name, entry, no_clone)?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "ok": true,
                            "name": name,
                            "materialized": outcome.map(|o| match o {
                                MaterializeOutcome::Cloned => "cloned",
                                MaterializeOutcome::AlreadyPresent => "already_present",
                            }),
                        }))?;
                    } else {
                        match outcome {
                            Some(MaterializeOutcome::Cloned) => {
                                println!("added project {name} (cloned)")
                            }
                            Some(MaterializeOutcome::AlreadyPresent) => {
                                println!("added project {name} (already present)")
                            }
                            None => println!("added project {name}"),
                        }
                    }
                    Ok(0)
                }
                ProjectCmd::Rm {
                    name,
                    delete,
                    force,
                } => {
                    project_rm(&git, &ws.root, &mut ws.config, &name, delete, force)?;
                    if out.json {
                        print_json(&serde_json::json!({ "ok": true, "name": name }))?;
                    } else {
                        println!("removed project {name}");
                    }
                    Ok(0)
                }
                ProjectCmd::Info { name } => {
                    let entry = ws.config.projects.get(&name).ok_or_else(|| {
                        OdmError::usage(format!("unknown project '{name}'"))
                    })?;
                    let snap = build_status(&ws)?;
                    let st = snap
                        .projects
                        .iter()
                        .find(|p| p.name == name)
                        .ok_or_else(|| OdmError::usage(format!("unknown project '{name}'")))?;
                    let origin = if st.is_git {
                        git.origin_url(&abs_checkout(&ws.root, &entry.path)).ok()
                    } else {
                        None
                    };
                    if out.json {
                        print_json(&serde_json::json!({
                            "name": name,
                            "path": entry.path,
                            "url": entry.url,
                            "branch": entry.branch,
                            "type": entry.type_,
                            "on_disk": st.on_disk,
                            "is_git": st.is_git,
                            "head": st.head,
                            "origin": origin,
                            "dirty": st.dirty,
                            "pin_rev": st.pin_rev,
                            "pin_state": st.pin_state,
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
                        println!("on_disk: {}", st.on_disk);
                        println!("is_git: {}", st.is_git);
                        if let Some(h) = &st.head {
                            println!("head: {h}");
                        }
                        if let Some(o) = &origin {
                            println!("origin: {o}");
                        }
                        println!("pin_state: {:?}", st.pin_state);
                    }
                    Ok(0)
                }
                ProjectCmd::Git {
                    name,
                    wt,
                    git_args,
                } => {
                    if wt.is_some() {
                        return Err(OdmError::not_implemented("project git --wt"));
                    }
                    let status = project_git(&git, &ws, &name, &git_args)?;
                    Ok(status.code().unwrap_or(1))
                }
            }
        }
    }
}

fn path_to_rel(path: &Path) -> Result<String, OdmError> {
    let s = path.to_string_lossy();
    if path.is_absolute() {
        return Err(OdmError::usage(format!(
            "project path must be relative, got '{s}'"
        )));
    }
    Ok(s.replace('\\', "/"))
}
