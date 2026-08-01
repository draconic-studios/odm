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

    /// Include/select Progen by config name (repeatable).
    #[arg(long = "progen", global = true)]
    pub progen: Vec<String>,

    /// Include members of a Progen group (repeatable).
    #[arg(long = "progen-group", global = true)]
    pub progen_group: Vec<String>,

    /// Project name for commands that target a project (e.g. run).
    #[arg(long, global = true)]
    pub project: Option<String>,

    /// Worktree slot name (with --project).
    #[arg(long, global = true)]
    pub wt: Option<String>,

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

    /// Progen lifecycle + store façade.
    Progen {
        #[command(subcommand)]
        cmd: ProgenCmd,
    },

    /// Federated vault search (FTS).
    Find {
        /// Free-text query (empty = list scoped notes).
        query: Option<String>,
    },

    /// In-store note neighborhood via wikilinks.
    Context {
        /// Note id, or `progen:id`.
        id: String,
    },

    /// List or run an Action from merged bundles.
    Run {
        /// Action name (omit to list).
        action: Option<String>,
        /// Extra args after `--`
        #[arg(last = true)]
        extra: Vec<String>,
    },

    /// Generate from a template (not implemented).
    Generate {
        /// Generator name.
        name: Option<String>,
    },

    /// Agent pack / session helpers (not implemented).
    Agent {
        #[command(subcommand)]
        cmd: AgentCmd,
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
    /// Worktree slot lifecycle.
    Worktree {
        #[command(subcommand)]
        cmd: ProjectWorktreeCmd,
    },
}

#[derive(Debug, Subcommand)]
pub enum ProjectWorktreeCmd {
    /// List worktree slots for a project.
    List {
        /// Project name.
        project: String,
    },
    /// Add a worktree slot.
    Add {
        /// Project name.
        project: String,
        /// Slot name (simple path segment).
        slot: String,
        /// Create and check out a new branch.
        #[arg(long)]
        branch: Option<String>,
    },
    /// Remove a worktree slot.
    Rm {
        /// Project name.
        project: String,
        /// Slot name.
        slot: String,
        /// Force remove (dirty worktree).
        #[arg(long)]
        force: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ProgenCmd {
    /// List configured progens.
    List,
    /// Add a progen (Obsidian-compatible vault path).
    Add {
        name: String,
        #[arg(long)]
        path: PathBuf,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long)]
        no_clone: bool,
    },
    /// Remove a progen entry.
    Rm {
        name: String,
        #[arg(long)]
        delete: bool,
        #[arg(long)]
        force: bool,
    },
    /// Show one progen / vault info.
    Info { name: String },
    /// Get a note by id (single-root).
    Get { id: String },
    /// Print note body only (single-root).
    Body { id: String },
    /// List note paths as a sorted tree (single-root).
    Tree,
    /// Notes that wikilink to id (single-root).
    Backlinks { id: String },
    /// List notes in a progen (single-root).
    Ls,
    /// Rebuild disposable index under `.odm/progen/<name>/`.
    Reindex,
    /// Store-side health (path + index).
    Doctor,
}

#[derive(Debug, Subcommand)]
pub enum AgentCmd {
    /// Install/link an agent pack (not implemented).
    Pack {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<String>,
    },
    /// Start an agent session (not implemented).
    Start {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<String>,
    },
    /// Prompt helpers (not implemented).
    Prompt {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        rest: Vec<String>,
    },
}
