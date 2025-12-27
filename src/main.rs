mod cli;
mod fs_ops;
mod web;
mod config;

use clap::Parser;
use cli::{Cli, Commands, NewTarget, ConfigAction};
use anyhow::Result;
use colored::*;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { target } => {
            match target {
                NewTarget::Folder { names } => {
                    for name in names {
                        fs_ops::create_folder(name)?;
                    }
                }
                NewTarget::File { names } => {
                    for name in names {
                        fs_ops::create_file(name)?;
                    }
                }
            }
        }
        Commands::Delete { paths, force } => {
            for path in paths {
                fs_ops::delete_item(path, *force)?;
            }
        }
        Commands::Rename { old_path, new_path } => {
            fs_ops::rename_item(old_path, new_path)?;
        }
        Commands::Move { source, destination } => {
            fs_ops::move_item(source, destination)?;
        }
        Commands::Search { query, open } => {
            web::search(query, *open)?;
        }
        Commands::Config { action } => {
            match action {
                ConfigAction::Init => config::init_config()?,
                ConfigAction::Where => config::show_config_location()?,
            }
        }
    }

    Ok(())
}