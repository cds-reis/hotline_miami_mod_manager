use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

use fs_extra::{copy_items_with_progress, dir::TransitProcessResult};
use indicatif::{ProgressBar, ProgressStyle};
use thiserror::Error;

use crate::{
    configs::paths_config::{ModsPath, ProgramPath},
    hotline_mod::{AssociatedMods, HotlineModName},
};

use super::update_progress_bar;

pub fn replace_mods(
    mods_path: &ModsPath,
    associated_mods: &AssociatedMods,
    mod_name: &HotlineModName,
) -> Result<(), ReplaceModError> {
    remove_mods_in_mods_dir(mods_path)?;

    if associated_mods.mods().is_empty() {
        return Ok(());
    }

    let progress_bar = progress_bar(mod_name);
    let handler = |transit_process: fs_extra::TransitProcess| {
        update_progress_bar(
            &progress_bar,
            transit_process.total_bytes,
            transit_process.copied_bytes,
        );
        TransitProcessResult::ContinueOrAbort
    };
    match copy_items_with_progress(
        associated_mods.mods(),
        mods_path.path(),
        &fs_extra::dir::CopyOptions::new(),
        handler,
    ) {
        Ok(_) => Ok(()),
        Err(error) => Err(ReplaceModError::FsExtra(error)),
    }
}

#[derive(Error, Debug)]
pub enum ReplaceModError {
    #[error("Error trying to copy the mods bytes.")]
    FsExtra(#[from] fs_extra::error::Error),
    #[error("Error reading the mods directory")]
    ReadingModsDirectory(io::Error),
    #[error("Error removing the mods directory")]
    DeletingModFile(io::Error, Box<Path>),
}

fn remove_mods_in_mods_dir(mods_path: &ModsPath) -> Result<(), ReplaceModError> {
    let files_to_remove = fs::read_dir(mods_path.path())
        .map_err(ReplaceModError::ReadingModsDirectory)?
        .filter_map(Result::ok)
        .filter(file_is_patchwad)
        .collect::<Vec<_>>();

    for file in files_to_remove {
        let path = file.path();
        fs::remove_file(&path)
            .map_err(|err| ReplaceModError::DeletingModFile(err, path.into_boxed_path()))?;
    }

    Ok(())
}

fn file_is_patchwad(dir_entry: &DirEntry) -> bool {
    let path = dir_entry.path();

    let is_file = path.is_file();
    let is_patchwad = path
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("patchwad"));

    is_file && is_patchwad
}

fn progress_bar(mod_name: &HotlineModName) -> ProgressBar {
    let progress_bar_message = format!("Copying {mod_name} mods.");
    let style = ProgressStyle::default_bar().template("{msg}").unwrap();
    ProgressBar::new(0)
        .with_message(progress_bar_message)
        .with_style(style)
}
