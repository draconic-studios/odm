mod cli;
mod output;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use odm::commands::{
    find_notes_dto, format_action_list_human, format_generate_run_human,
    format_generator_list_human, format_pack_install_human, format_pack_link_human,
    format_pack_list_human, format_pack_rm_human, format_progen_add_human, format_progen_info_human,
    format_progen_list_human, format_project_add_human, format_project_info_human,
    format_project_list_human, format_worktree_add_human, format_worktree_list_human,
    format_worktree_prune_all_human,
    format_worktree_prune_human, format_worktree_rm_human, list_actions_dto, list_generators_dto,
    list_projects, list_progens, materialize_json, materialize_json_opt, materialize_sync_human,
    pack_entry_dto, pack_list_dto, progen_info, project_info, status_snapshot, worktree_list_dto,
    worktree_prune_all_dto, worktree_prune_dto, worktree_slot_action_dto, GenerateRunDto,
};
use odm_actions::{run_action, CwdTarget, RunOptions, StdioMode};
use odm_core::{
    discover_root, format_doctor_human, format_status_human, generate_local, init_workspace,
    load_workspace, pack_install, pack_link, pack_list, pack_rm, path_buf_to_rel, pin_apply,
    pin_status, project_add, project_git, project_rm, run_doctor, sync_managed, worktree_add,
    worktree_list, worktree_prune, worktree_prune_all, worktree_rm, InitOptions, OdmError,
    ProgenEntry, ProjectEntry,
};
use odm_git::Git;
use odm_progen::{
    add_progen, context_notes, doctor_progens, format_context_human, format_find_human,
    format_get_human, format_ls_human, get_note, list_notes, one_progen_flag, open_for_id,
    open_single, reindex_for_cli, rm_progen,
};

use cli::{AgentCmd, Cli, Commands, PackCmd, PinCmd, ProgenCmd, ProjectCmd, ProjectWorktreeCmd};
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
                            "materialized": materialize_json(r.materialized),
                            "fetched": r.fetched,
                            "head": r.head,
                        })
                    }).collect::<Vec<_>>(),
                }))?;
            } else if results.is_empty() {
                println!("(no managed entries)");
            } else {
                for r in &results {
                    println!(
                        "{}\t{}\tfetched",
                        r.name,
                        materialize_sync_human(r.materialized)
                    );
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
            let git = Git::new();
            let snap = status_snapshot(&git, &ws)?;
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
            let git = Git::new();
            let report = run_doctor(&git, &ws, fix)?;
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
                        print_json(&list_projects(&git, &ws)?)?;
                    } else {
                        let snap = status_snapshot(&git, &ws)?;
                        print!("{}", format_project_list_human(&ws, &snap));
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
                    let rel = path_buf_to_rel(&path)?;
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
                            "materialized": materialize_json_opt(outcome),
                        }))?;
                    } else {
                        println!("{}", format_project_add_human(&name, outcome));
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
                    let dto = project_info(&git, &ws, &name)?;
                    if out.json {
                        print_json(&dto)?;
                    } else {
                        print!("{}", format_project_info_human(&dto));
                    }
                    Ok(0)
                }
                ProjectCmd::Git {
                    name,
                    wt,
                    git_args,
                } => {
                    let effective_wt = wt.or(global_wt);
                    let status =
                        project_git(&git, &ws, &name, &git_args, effective_wt.as_deref())?;
                    Ok(status.code().unwrap_or(1))
                }
                ProjectCmd::Worktree { cmd } => match cmd {
                    ProjectWorktreeCmd::List { project } => {
                        let outcome = worktree_list(&git, &ws, &project)?;
                        if out.json {
                            print_json(&worktree_list_dto(&outcome))?;
                        } else {
                            print!("{}", format_worktree_list_human(&outcome));
                        }
                        Ok(0)
                    }
                    ProjectWorktreeCmd::Add {
                        project,
                        slot,
                        branch,
                    } => {
                        let outcome =
                            worktree_add(&git, &ws, &project, &slot, branch.as_deref())?;
                        if out.json {
                            print_json(&worktree_slot_action_dto(&outcome))?;
                        } else {
                            println!("{}", format_worktree_add_human(&outcome));
                        }
                        Ok(0)
                    }
                    ProjectWorktreeCmd::Rm {
                        project,
                        slot,
                        force,
                    } => {
                        let outcome = worktree_rm(&git, &ws, &project, &slot, force)?;
                        if out.json {
                            print_json(&worktree_slot_action_dto(&outcome))?;
                        } else {
                            println!("{}", format_worktree_rm_human(&outcome));
                        }
                        Ok(0)
                    }
                    ProjectWorktreeCmd::Prune {
                        project,
                        all,
                        force,
                    } => {
                        if all {
                            let outcome = worktree_prune_all(&git, &ws, force)?;
                            if out.json {
                                print_json(&worktree_prune_all_dto(&outcome))?;
                            } else {
                                println!("{}", format_worktree_prune_all_human(&outcome));
                            }
                            if outcome.skipped_nonempty.is_empty() {
                                Ok(0)
                            } else {
                                Ok(3)
                            }
                        } else {
                            let project = project.expect("clap requires project unless --all");
                            let outcome = worktree_prune(&git, &ws, &project, force)?;
                            if out.json {
                                print_json(&worktree_prune_dto(&outcome))?;
                            } else {
                                println!("{}", format_worktree_prune_human(&outcome));
                            }
                            // Partial: empties removed; non-empty orphans remain → exit 3 after output.
                            if outcome.skipped_nonempty.is_empty() {
                                Ok(0)
                            } else {
                                Ok(3)
                            }
                        }
                    }
                },
            }
        }
        Commands::Progen { cmd } => run_progen(cli.root.as_deref(), &cli.progen, out, cmd),
        Commands::Find { query, limit } => {
            if limit == 0 {
                return Err(OdmError::usage("--limit must be at least 1"));
            }
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            let q = query.unwrap_or_default();
            let dto = find_notes_dto(&ws, &q, &cli.progen, &cli.progen_group, limit)?;
            if out.json {
                print_json(&dto)?;
            } else {
                print!("{}", format_find_human(&dto.hits));
            }
            Ok(0)
        }
        Commands::Context { id } => run_context_prompt(
            cli.root.as_deref(),
            &cli.progen,
            out,
            &id,
            "context accepts at most one --progen (or use name:id)",
        ),
        Commands::Run { action, extra } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            match action {
                None => {
                    let dto = list_actions_dto(&ws);
                    if out.json {
                        print_json(&dto)?;
                    } else {
                        print!("{}", format_action_list_human(&dto));
                    }
                    Ok(0)
                }
                Some(name) => {
                    let cwd =
                        CwdTarget::from_flags(global_project.as_deref(), global_wt.as_deref())?;
                    let stdio = if out.json {
                        StdioMode::Capture
                    } else {
                        StdioMode::Inherit
                    };
                    let result = run_action(
                        &ws,
                        &name,
                        RunOptions {
                            cwd,
                            extra_args: &extra,
                            stdio,
                        },
                    )?;
                    if out.json {
                        print_json(&serde_json::json!({
                            "action": name,
                            "exitCode": result.exit_code,
                        }))?;
                    }
                    Ok(result.exit_code)
                }
            }
        }
        Commands::Generate {
            name,
            dest,
            force,
            dry_run,
        } => {
            let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
            let ws = load_workspace(&root)?;
            match name {
                None => {
                    let dto = list_generators_dto(&ws);
                    if out.json {
                        print_json(&dto)?;
                    } else {
                        print!("{}", format_generator_list_human(&dto));
                    }
                    Ok(0)
                }
                Some(name) => {
                    let dest = dest.ok_or_else(|| {
                        OdmError::usage("generate requires --dest <path> when a name is given")
                    })?;
                    let dest_rel = path_buf_to_rel(&dest)?;
                    let outcome = generate_local(&ws, &name, &dest_rel, force, dry_run)?;
                    if out.json {
                        print_json(&GenerateRunDto {
                            generator: name,
                            dest: dest_rel,
                            copied: outcome.copied,
                            dry_run,
                        })?;
                    } else {
                        print!(
                            "{}",
                            format_generate_run_human(&name, &dest_rel, outcome.copied, dry_run)
                        );
                    }
                    Ok(0)
                }
            }
        }
        Commands::Agent { cmd } => match cmd {
            AgentCmd::Pack { cmd } => {
                let root = discover_root(cli.root.as_deref(), &std::env::current_dir()?)?;
                let ws = load_workspace(&root)?;
                match cmd {
                    PackCmd::List => {
                        let entries = pack_list(&ws)?;
                        let dto = pack_list_dto(&entries);
                        if out.json {
                            print_json(&dto)?;
                        } else {
                            print!("{}", format_pack_list_human(&dto));
                        }
                        Ok(0)
                    }
                    PackCmd::Install { source, home, force } => {
                        let entry = pack_install(&ws, &source, &home, force)?;
                        if out.json {
                            print_json(&pack_entry_dto(&entry))?;
                        } else {
                            print!("{}", format_pack_install_human(&entry));
                        }
                        Ok(0)
                    }
                    PackCmd::Link { source, home, force } => {
                        let entry = pack_link(&ws, &source, &home, force)?;
                        if out.json {
                            print_json(&pack_entry_dto(&entry))?;
                        } else {
                            print!("{}", format_pack_link_human(&entry));
                        }
                        Ok(0)
                    }
                    PackCmd::Rm { name } => {
                        let entry = pack_rm(&ws, &name)?;
                        if out.json {
                            print_json(&pack_entry_dto(&entry))?;
                        } else {
                            print!("{}", format_pack_rm_human(&entry));
                        }
                        Ok(0)
                    }
                }
            }
            AgentCmd::Start { .. } => Err(OdmError::not_implemented("agent start")),
            AgentCmd::Prompt { id } => run_context_prompt(
                cli.root.as_deref(),
                &cli.progen,
                out,
                &id,
                "agent prompt accepts at most one --progen (or use name:id)",
            ),
        },
    }
}

/// Shared path for `odm context` and `odm agent prompt` (thin context packaging).
fn run_context_prompt(
    root_flag: Option<&Path>,
    global_progen: &[String],
    out: &GlobalOut,
    id: &str,
    multi_progen_msg: &str,
) -> Result<i32, OdmError> {
    let root = discover_root(root_flag, &std::env::current_dir()?)?;
    let ws = load_workspace(&root)?;
    let progen = one_progen_flag(global_progen, multi_progen_msg)?;
    let hit = context_notes(&ws, id, progen)?;
    if out.json {
        print_json(&hit)?;
    } else {
        print!("{}", format_context_human(&hit));
    }
    Ok(0)
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

    match cmd {
        ProgenCmd::List => {
            if out.json {
                print_json(&list_progens(&git, &ws)?)?;
            } else {
                let snap = status_snapshot(&git, &ws)?;
                print!("{}", format_progen_list_human(&ws, &snap));
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
            let outcome = add_progen(&git, &ws.root, &mut ws.config, &name, entry, no_clone)?;
            if out.json {
                print_json(&serde_json::json!({
                    "ok": true,
                    "name": name,
                    "materialized": materialize_json_opt(outcome),
                }))?;
            } else {
                println!("{}", format_progen_add_human(&name, outcome));
            }
            Ok(0)
        }
        ProgenCmd::Rm {
            name,
            delete,
            force,
        } => {
            rm_progen(&git, &ws.root, &mut ws.config, &name, delete, force)?;
            if out.json {
                print_json(&serde_json::json!({ "ok": true, "name": name }))?;
            } else {
                println!("removed progen {name}");
            }
            Ok(0)
        }
        ProgenCmd::Info { name } => {
            let dto = progen_info(&ws, &name)?;
            if out.json {
                print_json(&dto)?;
            } else {
                print!("{}", format_progen_info_human(&dto));
            }
            Ok(0)
        }
        ProgenCmd::Get { id } => {
            let progen = one_progen_flag(
                global_progen,
                "progen get accepts at most one --progen (or use name:id)",
            )?;
            let g = get_note(&ws, &id, progen)?;
            if out.json {
                print_json(&g)?;
            } else {
                print!("{}", format_get_human(&g));
            }
            Ok(0)
        }
        ProgenCmd::Body { id } => {
            let progen = one_progen_flag(
                global_progen,
                "progen body accepts at most one --progen (or use name:id)",
            )?;
            // Body is presentation of get (reduced JSON / body stdout).
            let g = get_note(&ws, &id, progen)?;
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
            let progen =
                one_progen_flag(global_progen, "progen tree accepts at most one --progen")?;
            let paths = open_single(&ws, progen)?.tree()?;
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
            let progen = one_progen_flag(
                global_progen,
                "progen backlinks accepts at most one --progen (or use name:id)",
            )?;
            let (store, nid) = open_for_id(&ws, &id, progen)?;
            let hits = store.backlinks(&nid)?;
            if out.json {
                print_json(&serde_json::json!({ "backlinks": hits }))?;
            } else {
                print!("{}", format_ls_human(&hits));
            }
            Ok(0)
        }
        ProgenCmd::Ls => {
            let progen =
                one_progen_flag(global_progen, "progen ls accepts at most one --progen")?;
            let hits = list_notes(&ws, progen)?;
            if out.json {
                print_json(&serde_json::json!({ "notes": hits }))?;
            } else {
                print!("{}", format_ls_human(&hits));
            }
            Ok(0)
        }
        ProgenCmd::Reindex => {
            let stats = reindex_for_cli(&ws, global_progen)?;
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
            let progen =
                one_progen_flag(global_progen, "progen doctor accepts at most one --progen")?;
            let checks = doctor_progens(&ws, progen)?;
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
