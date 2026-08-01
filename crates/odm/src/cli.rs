use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "odm", version, about = "Orchestrated Development Management")]
pub struct Cli {
    /// Workspace root (must contain `.odm/odm.config.yaml`). No upward walk.
    #[arg(long, global = true)]
    pub root: Option<PathBuf>,

    /// Machine-readable JSON on stdout.
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Bootstrap a new Workspace.
    Init {
        /// Target path (default: cwd).
        path: Option<PathBuf>,

        /// Skip `git init` at Workspace root.
        #[arg(long)]
        no_git: bool,

        /// Interactive prompts (not implemented day one).
        #[arg(short = 'i', long)]
        interactive: bool,
    },

    /// Materialize managed entries and fetch (not checkout).
    Sync {
        /// Entity names (default: all managed).
        names: Vec<String>,
    },

    /// Pin file apply / status.
    Pin {
        #[command(subcommand)]
        cmd: PinCmd,
    },

    /// Workspace snapshot.
    Status,

    /// ODM-side health checks.
    Doctor {
        /// Mechanical repairs only.
        #[arg(long)]
        fix: bool,
    },

    /// Project lifecycle.
    Project {
        #[command(subcommand)]
        cmd: ProjectCmd,
    },
}

#[derive(Debug, Subcommand)]
pub enum PinCmd {
    /// Checkout pinned revs as detached HEAD.
    Apply {
        names: Vec<String>,
        #[arg(long)]
        force: bool,
    },
    /// Compare pin file vs current HEAD.
    Status { names: Vec<String> },
}

#[derive(Debug, Subcommand)]
pub enum ProjectCmd {
    /// List configured projects.
    List,
    /// Add a project entry.
    Add {
        name: String,
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long = "type")]
        type_: Option<String>,
        #[arg(long)]
        no_clone: bool,
    },
    /// Remove a project entry.
    Rm {
        name: String,
        #[arg(long)]
        delete: bool,
        #[arg(long)]
        force: bool,
    },
    /// Show one project.
    Info { name: String },
    /// Run git in a project checkout.
    Git {
        name: String,
        #[arg(long)]
        wt: Option<String>,
        #[arg(last = true)]
        git_args: Vec<String>,
    },
}
