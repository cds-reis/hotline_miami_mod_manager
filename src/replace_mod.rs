use std::{
    fs::{create_dir, remove_dir_all},
    path::Path,
};

use fs_extra::dir::{self, TransitProcessResult};
use fs_extra::{
    copy_items_with_progress,
    file::{copy_with_progress, CopyOptions, TransitProcess},
};
use indicatif::ProgressBar;

use crate::{configs::Configs, hotline_mod::HotlineMod, list_mods::format_mod_name};

pub fn replace_mod(hm_mod: &HotlineMod, config: &Configs) {
    if let Some(music_path) = &hm_mod.music {
        replace_music(music_path, config);
    }
    replace_mods(hm_mod, config);
}

fn replace_mods(hm_mod: &HotlineMod, config: &Configs) {
    let game_mods_path = &config.paths_config.mods_path;
    remove_mods_in_mods_dir(game_mods_path);
    if hm_mod.mods.is_empty() {
        return;
    }
    let music = hm_mod.music.as_deref();
    let progress_bar = ProgressBar::new(0).with_message(format_progress_bar_mods_message(music));
    let handler = |transit_process: fs_extra::TransitProcess| {
        progress_bar.set_length(transit_process.total_bytes);
        progress_bar.set_position(transit_process.copied_bytes);
        TransitProcessResult::ContinueOrAbort
    };
    copy_items_with_progress(
        &hm_mod.mods,
        game_mods_path,
        &dir::CopyOptions::new(),
        handler,
    )
    .expect("Could not copy your mods successfully.");
}

fn remove_mods_in_mods_dir(mods_path: &Path) {
    remove_dir_all(mods_path).expect("Error removing mods in the mods directory");
    create_dir(mods_path).expect("Error removing mods in the mods directory.");
}

fn replace_music(music_path: &Path, config: &Configs) {
    let game_music_path = &config.paths_config.game_path.join(MUSIC_FILE_NAME);
    let copy_options = default_copy_options();
    let message = format_progress_bar_music_message(music_path);
    let progress_bar = ProgressBar::new(0).with_message(message);
    let handler = |transit_process: TransitProcess| {
        progress_bar.set_length(transit_process.total_bytes);
        progress_bar.set_position(transit_process.copied_bytes);
    };
    let _ = copy_with_progress(music_path, game_music_path, &copy_options, handler)
        .expect("Could not copy the music file successfully.");
}

fn format_progress_bar_music_message(music_path: &Path) -> String {
    music_path.file_name().map(format_mod_name).map_or_else(
        || String::from("Copying music file."),
        |name| format!("Copying {} music.", name),
    )
}

fn format_progress_bar_mods_message(music_path: Option<&Path>) -> String {
    music_path
        .and_then(Path::file_name)
        .map(format_mod_name)
        .map_or_else(
            || String::from("Copying mods files."),
            |name| format!("Copying {} mods.", name),
        )
}

fn default_copy_options() -> CopyOptions {
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options
}

const MUSIC_FILE_NAME: &str = "hlm2_music_desktop.wad";
