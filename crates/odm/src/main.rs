mod cli;
mod output;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use odm_actions::{list_actions, run_action, RunOptions};
use odm_core::{
    abs_checkout, build_status, discover_root, format_doctor_human, format_status_human,
    init_workspace, load_workspace, path_buf_to_rel, pin_apply, pin_status, progen_add, progen_rm,
    project_add, project_git, project_rm, run_doctor, sync_managed, InitOptions, MaterializeOutcome,
    OdmError, PinState, ProgenEntry, ProjectEntry,
};
use odm_git::Git;
use odm_progen::{
    context_notes, doctor_progens, ensure_vault, find_notes, format_context_human,
    format_find_human, format_get_human, format_ls_human, get_note, list_notes, note_backlinks,
    note_body, note_tree, reindex_progen, resolve_read_scope, resolve_single_read, vault_info,
    ScopedProgen,
};

use cli::{AgentCmd, Cli, Commands, PinCmd, ProgenCmd, ProjectCmd};
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
    let global_project = cli.project.clone();
    let global_wt = cli.wt.clone();
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
                            println!(
                                "{}\t{}\t{}",
                                r.name,
                                r.status,
                                r.rev.as_deref().unwrap_or("-")
                            );
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
                    let snap = build_status(&ws)?;
                    if out.json {
                        let list: Vec<_> = ws
                            .config
                            .projects
                            .iter()
                            .map(|(name, e)| {
                                let st = snap.projects.iter().find(|p| p.name == *name);
                                serde_json::json!({
                                    "name": name,
                                    "path": e.path,
                                    "url": e.url,
                                    "branch": e.branch,
                                    "type": e.type_,
                                    "on_disk": st.map(|s| s.on_disk).unwrap_or(false),
                                    "is_git": st.map(|s| s.is_git).unwrap_or(false),
                                    "pin_state": st.map(|s| s.pin_state),
                                })
                            })
                            .collect();
                        print_json(&serde_json::json!({ "projects": list }))?;
                    } else if ws.config.projects.is_empty() {
                        println!("(no projects)");
                    } else {
                        for (name, e) in &ws.config.projects {
                            let managed = if e.url.is_some() { "managed" } else { "path" };
                            let st = snap.projects.iter().find(|p| p.name == *name);
                            let on_disk = st.map(|s| s.on_disk).unwrap_or(false);
                            let is_git = st.map(|s| s.is_git).unwrap_or(false);
                            let pin = st.map(|s| pin_state_label(s.pin_state)).unwrap_or("-");
                            println!(
                                "{name}\t{}\t{managed}\ton_disk={on_disk}\tis_git={is_git}\tpin={pin}",
                                e.path
                            );
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
                    let effective_wt = wt.or(global_wt);
                    if effective_wt.is_some() {
                        return Err(OdmError::not_implemented("project git --wt"));
                    }
                    let status = project_git(&git, &ws, &name, &git_args)?;
                    Ok(status.code().unwrap_or(1))
                }
                ProjectCmd::Worktree { .. } => Err(OdmError::not_implemented("project worktree")),
            }
        }
        Commands::Progen { cmd } => run_progen(cli.root.as_deref(), &cli.progen, out, cmd),
        Commands::Find { query } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let q = query.unwrap_or_default();
            let hits = find_notes(&ws, &q, &cli.progen, &cli.progen_group, 200)?;
            if out.json {
                print_json(&serde_json::json!({ "hits": hits }))?;
            } else {
                print!("{}", format_find_human(&hits));
            }
            Ok(0)
        }
        Commands::Context { id } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let progen = cli.progen.first().map(|s| s.as_str());
            if cli.progen.len() > 1 {
                return Err(OdmError::usage(
                    "context accepts at most one --progen (or use name:id)",
                ));
            }
            let hit = context_notes(&ws, &id, progen)?;
            if out.json {
                print_json(&hit)?;
            } else {
                print!("{}", format_context_human(&hit));
            }
            Ok(0)
        }
        Commands::Run { action, extra } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            match action {
                None => {
                    let listed = list_actions(&ws);
                    if out.json {
                        let actions: Vec<_> = listed
                            .iter()
                            .map(|(name, def)| {
                                serde_json::json!({
                                    "name": name,
                                    "tasks": def.tasks.iter().map(|t| {
                                        serde_json::json!({
                                            "run": t.run,
                                            "dir": t.dir,
                                        })
                                    }).collect::<Vec<_>>(),
                                })
                            })
                            .collect();
                        print_json(&serde_json::json!({ "actions": actions }))?;
                    } else if listed.is_empty() {
                        println!("(no actions)");
                    } else {
                        for (name, _) in listed {
                            println!("{name}");
                        }
                    }
                    Ok(0)
                }
                Some(name) => {
                    let code = run_action(
                        &ws,
                        &name,
                        RunOptions {
                            project: global_project.as_deref(),
                            wt: global_wt.as_deref(),
                            extra_args: &extra,
                        },
                    )?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "action": name,
                            "exitCode": code,
                        }))?;
                    }
                    Ok(code)
                }
            }
        }
        Commands::Generate { .. } => Err(OdmError::not_implemented("generate")),
        Commands::Agent { cmd } => {
            let verb = match cmd {
                AgentCmd::Pack { .. } => "agent pack",
                AgentCmd::Start { .. } => "agent start",
                AgentCmd::Prompt { .. } => "agent prompt",
            };
            Err(OdmError::not_implemented(verb))
        }
    }
}

fn run_progen(
    root_flag: Option<&Path>,
    global_progen: &[String],
    out: &GlobalOut,
    cmd: ProgenCmd,
) -> Result<i32, OdmError> {
    let root = discover_root(root_flag, &std::env::current_dir()?)?;
    let mut ws = load_workspace(&root)?;
    let git = Git::new();
    let progen_flag = global_progen.first().map(|s| s.as_str());

    match cmd {
        ProgenCmd::List => {
            let snap = build_status(&ws)?;
            if out.json {
                let list: Vec<_> = ws
                    .config
                    .progens
                    .iter()
                    .map(|(name, e)| {
                        let st = snap.progens.iter().find(|p| p.name == *name);
                        serde_json::json!({
                            "name": name,
                            "path": e.path,
                            "url": e.url,
                            "branch": e.branch,
                            "on_disk": st.map(|s| s.on_disk).unwrap_or(false),
                            "is_git": st.map(|s| s.is_git).unwrap_or(false),
                            "pin_state": st.map(|s| s.pin_state),
                        })
                    })
                    .collect();
                print_json(&serde_json::json!({ "progens": list }))?;
            } else if ws.config.progens.is_empty() {
                println!("(no progens)");
            } else {
                for (name, e) in &ws.config.progens {
                    let managed = if e.url.is_some() { "managed" } else { "path" };
                    let st = snap.progens.iter().find(|p| p.name == *name);
                    let on_disk = st.map(|s| s.on_disk).unwrap_or(false);
                    let is_git = st.map(|s| s.is_git).unwrap_or(false);
                    let pin = st.map(|s| pin_state_label(s.pin_state)).unwrap_or("-");
                    println!(
                        "{name}\t{}\t{managed}\ton_disk={on_disk}\tis_git={is_git}\tpin={pin}",
                        e.path
                    );
                }
            }
            Ok(0)
        }
        ProgenCmd::Add {
            name,
            path,
            url,
            branch,
            no_clone,
        } => {
            let rel = path_buf_to_rel(&path)?;
            let entry = ProgenEntry {
                path: rel,
                url,
                branch,
            };
            let outcome = progen_add(
                &git,
                &ws.root,
                &mut ws.config,
                &name,
                entry,
                no_clone,
                ensure_vault,
            )?;
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
                        println!("added progen {name} (cloned vault)")
                    }
                    Some(MaterializeOutcome::AlreadyPresent) => {
                        println!("added progen {name} (already present)")
                    }
                    None => println!("added progen {name} (vault ready)"),
                }
            }
            Ok(0)
        }
        ProgenCmd::Rm {
            name,
            delete,
            force,
        } => {
            progen_rm(&git, &ws.root, &mut ws.config, &name, delete, force)?;
            if out.json {
                print_json(&serde_json::json!({ "ok": true, "name": name }))?;
            } else {
                println!("removed progen {name}");
            }
            Ok(0)
        }
        ProgenCmd::Info { name } => {
            let entry = ws
                .config
                .progens
                .get(&name)
                .ok_or_else(|| OdmError::usage(format!("unknown progen '{name}'")))?;
            let sp = ScopedProgen {
                name: name.clone(),
                path: abs_checkout(&ws.root, &entry.path),
            };
            let info = vault_info(&sp)?;
            if out.json {
                print_json(&serde_json::json!({
                    "name": name,
                    "path": entry.path,
                    "url": entry.url,
                    "branch": entry.branch,
                    "on_disk": info.on_disk,
                    "note_count": info.note_count,
                    "has_obsidian": info.has_obsidian,
                    "abs_path": info.path,
                }))?;
            } else {
                println!("name: {name}");
                println!("path: {}", entry.path);
                if let Some(u) = &entry.url {
                    println!("url: {u}");
                }
                println!("on_disk: {}", info.on_disk);
                println!("notes: {}", info.note_count);
                println!("obsidian: {}", info.has_obsidian);
                println!("abs: {}", info.path.display());
            }
            Ok(0)
        }
        ProgenCmd::Get { id } => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage(
                    "progen get accepts at most one --progen (or use name:id)",
                ));
            }
            let g = get_note(&ws, &id, progen_flag)?;
            if out.json {
                print_json(&g)?;
            } else {
                print!("{}", format_get_human(&g));
            }
            Ok(0)
        }
        ProgenCmd::Body { id } => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage(
                    "progen body accepts at most one --progen (or use name:id)",
                ));
            }
            let g = note_body(&ws, &id, progen_flag)?;
            if out.json {
                print_json(&serde_json::json!({
                    "progen": g.progen,
                    "id": g.id,
                    "body": g.body,
                }))?;
            } else {
                print!("{}", g.body);
                if !g.body.ends_with('\n') {
                    println!();
                }
            }
            Ok(0)
        }
        ProgenCmd::Tree => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage("progen tree accepts at most one --progen"));
            }
            let paths = note_tree(&ws, progen_flag)?;
            if out.json {
                print_json(&serde_json::json!({ "paths": paths }))?;
            } else if paths.is_empty() {
                println!("(no notes)");
            } else {
                for p in paths {
                    println!("{p}");
                }
            }
            Ok(0)
        }
        ProgenCmd::Backlinks { id } => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage(
                    "progen backlinks accepts at most one --progen (or use name:id)",
                ));
            }
            let hits = note_backlinks(&ws, &id, progen_flag)?;
            if out.json {
                print_json(&serde_json::json!({ "backlinks": hits }))?;
            } else {
                print!("{}", format_ls_human(&hits));
            }
            Ok(0)
        }
        ProgenCmd::Ls => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage("progen ls accepts at most one --progen"));
            }
            let hits = list_notes(&ws, progen_flag)?;
            if out.json {
                print_json(&serde_json::json!({ "notes": hits }))?;
            } else {
                print!("{}", format_ls_human(&hits));
            }
            Ok(0)
        }
        ProgenCmd::Reindex => {
            let scope = if let Some(n) = progen_flag {
                if global_progen.len() > 1 {
                    return Err(OdmError::usage(
                        "progen reindex: pass one --progen or none for all",
                    ));
                }
                vec![resolve_single_read(&ws, Some(n))?]
            } else {
                resolve_read_scope(&ws, &[], &[])?
            };
            let mut stats = Vec::new();
            for sp in scope {
                stats.push(reindex_progen(&ws, &sp)?);
            }
            if out.json {
                print_json(&serde_json::json!({ "results": stats.iter().map(|s| {
                    serde_json::json!({
                        "progen": s.progen,
                        "notes": s.notes,
                        "links": s.links,
                    })
                }).collect::<Vec<_>>() }))?;
            } else {
                for s in stats {
                    println!("{}\t{} notes\t{} links", s.progen, s.notes, s.links);
                }
            }
            Ok(0)
        }
        ProgenCmd::Doctor => {
            if global_progen.len() > 1 {
                return Err(OdmError::usage("progen doctor accepts at most one --progen"));
            }
            let checks = doctor_progens(&ws, progen_flag)?;
            let ok = checks.iter().all(|c| c.ok);
            if out.json {
                print_json(&serde_json::json!({ "ok": ok, "checks": checks }))?;
            } else if checks.is_empty() {
                println!("(no progens)");
            } else {
                for c in &checks {
                    let mark = if c.ok { "ok" } else { "FAIL" };
                    println!("{}\t{}\t{}\t{}", c.progen, c.id, mark, c.message);
                }
            }
            if ok {
                Ok(0)
            } else {
                Ok(3)
            }
        }
    }
}

fn path_to_rel(path: &Path) -> Result<String, OdmError> {
    path_buf_to_rel(path)
}

fn pin_state_label(s: PinState) -> &'static str {
    match s {
        PinState::None => "none",
        PinState::MissingPath => "missing_path",
        PinState::Unpinned => "unpinned",
        PinState::InSync => "in_sync",
        PinState::Drift => "drift",
        PinState::MissingPinFile => "missing_pin_file",
    }
}
