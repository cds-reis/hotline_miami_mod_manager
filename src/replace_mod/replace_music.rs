use fs_extra::file::{copy_with_progress, CopyOptions, TransitProcess};
use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    configs::paths_config::{GamePath, ProgramPath},
    hotline_mod::{HotlineModName, Music},
};

use super::update_progress_bar;

pub fn replace_music(
    game_path: &GamePath,
    music: &Music,
    mod_name: &HotlineModName,
) -> Result<(), ReplaceMusicError> {
    let copy_options = default_copy_options();
    let game_music_path = game_path.path().join(MUSIC_FILE_NAME);
    let progress_bar = progress_bar(mod_name);
    let handler = |transit_process: TransitProcess| {
        update_progress_bar(
            &progress_bar,
            transit_process.total_bytes,
            transit_process.copied_bytes,
        );
    };

    match copy_with_progress(music, game_music_path, &copy_options, handler) {
        Ok(_) => Ok(()),
        Err(error) => Err(ReplaceMusicError::from(error)),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ReplaceMusicError {
    #[error("Error trying to copy the music bytes.")]
    FsExtraError(#[from] fs_extra::error::Error),
}

fn progress_bar(mod_name: &HotlineModName) -> ProgressBar {
    let progress_bar_message = format!("Copying {mod_name} music.");
    let style = ProgressStyle::default_bar().template("{msg}").unwrap();
    ProgressBar::new(0)
        .with_message(progress_bar_message)
        .with_style(style)
}

fn default_copy_options() -> CopyOptions {
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options
}

const MUSIC_FILE_NAME: &str = "hlm2_music_desktop.wad";
