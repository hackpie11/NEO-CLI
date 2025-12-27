use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "neo")]
#[command(about = "The terminal, but smarter.", long_about = None)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create new files or directories
    New {
        #[command(subcommand)]
        target: NewTarget,
    },
    /// Delete files or directories
    Delete {
        /// Paths to remove
        #[arg(required = true)]
        paths: Vec<String>,
        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Rename a file or directory
    Rename {
        /// Current path
        old_path: String,
        /// New path
        new_path: String,
    },
    /// Move a file or directory
    Move {
        /// Source path
        source: String,
        /// Destination path
        destination: String,
    },
    /// Search the web
    Search {
        /// The search query
        #[arg(required = true)]
        query: Vec<String>,
        
        /// Open top result directly in browser
        #[arg(short, long)]
        open: bool,
    },
    /// Configure Neo
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    }
}

#[derive(Subcommand)]
pub enum NewTarget {
    /// Create new folders
    Folder {
        /// Names of the folders
        #[arg(required = true)]
        names: Vec<String>,
    },
    /// Create new files
    File {
        /// Names of the files
        #[arg(required = true)]
        names: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Initialize default config
    Init,
    /// Show current config location
    Where,
}
