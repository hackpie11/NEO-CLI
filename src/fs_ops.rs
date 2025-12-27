use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use fs_extra::dir;
use dialoguer::Confirm;

pub fn create_folder(name: &str) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    spinner.set_message(format!("Creating folder '{}'...", name));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    fs::create_dir_all(name).context("Failed to create directory")?;

    spinner.finish_with_message(format!("{} Folder '{}' created successfully.", "✔".green(), name));
    Ok(())
}

pub fn create_file(name: &str) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}")?);
    spinner.set_message(format!("Creating file '{}'...", name));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    fs::File::create(name).context("Failed to create file")?;

    spinner.finish_with_message(format!("{} File '{}' created successfully.", "✔".green(), name));
    Ok(())
}

pub fn delete_item(path: &str, force: bool) -> Result<()> {
    if !force {
        let confirmation = Confirm::new()
            .with_prompt(format!("Are you sure you want to delete '{}'?", path.red()))
            .interact()?;

        if !confirmation {
            println!("{}", "Operation cancelled.".yellow());
            return Ok(());
        }
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.red} {msg}")?);
    spinner.set_message(format!("Deleting '{}'...", path));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let p = Path::new(path);
    if p.is_dir() {
        fs::remove_dir_all(p).context("Failed to remove directory")?;
    } else {
        fs::remove_file(p).context("Failed to remove file")?;
    }

    spinner.finish_with_message(format!("{} Item '{}' deleted.", "✔".green(), path));
    Ok(())
}

pub fn rename_item(old: &str, new: &str) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} {msg}")?);
    spinner.set_message(format!("Renaming '{}' to '{}'...", old, new));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    fs::rename(old, new).context("Failed to rename item")?;

    spinner.finish_with_message(format!("{} Renamed '{}' to '{}'.", "✔".green(), old, new));
    Ok(())
}

pub fn move_item(source: &str, destination: &str) -> Result<()> {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(ProgressStyle::default_spinner().template("{spinner:.blue} {msg}")?);
    spinner.set_message(format!("Moving '{}' to '{}'...", source, destination));
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let src_path = Path::new(source);
    let dest_path = Path::new(destination);

    // Basic move logic using fs_extra for directories or std::fs for files across filesystems if needed
    // Simple rename often works for move on same fs, but let's be robust
    if src_path.is_dir() {
        let options = dir::CopyOptions::new(); // Defaults are usually fine for move (copy+delete)
        fs_extra::dir::move_dir(src_path, dest_path, &options).context("Failed to move directory")?;
    } else {
        // fs::rename handles moves on same mount, copy/delete otherwise. 
        // For simplicity in this v1, we try rename first.
        match fs::rename(source, destination) {
            Ok(_) => {},
            Err(_) => {
                // Fallback to copy and delete if rename fails (e.g. cross-device)
                fs::copy(source, destination).context("Failed to copy file")?;
                fs::remove_file(source).context("Failed to remove source file after copy")?;
            }
        }
    }

    spinner.finish_with_message(format!("{} Moved '{}' to '{}'.", "✔".green(), source, destination));
    Ok(())
}
