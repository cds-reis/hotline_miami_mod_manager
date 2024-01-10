use std::{
    fs::{create_dir, remove_dir_all},
    path::Path,
};

use fs_extra::dir::{self, TransitProcessResult};
use fs_extra::{
    copy_items_with_progress,
    file::{copy_with_progress, CopyOptions, TransitProcess},
};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};

use crate::{
    configs::Configs, hotline_mod::HotlineMod, replace_default_music::replace_default_music,
};

pub fn replace_mod(hm_mod: &HotlineMod, config: &Configs) {
    match &hm_mod.music {
        Some(_) => replace_music(hm_mod, config),
        None => replace_default_music(config),
    }
    replace_mods(hm_mod, config);
}

fn replace_mods(hm_mod: &HotlineMod, config: &Configs) {
    let game_mods_path = &config.paths_config.mods_path;
    remove_mods_in_mods_dir(game_mods_path);
    if hm_mod.mods.is_empty() {
        return;
    }
    let message = format_progress_bar_mods_message(hm_mod);
    let progress_bar = ProgressBar::new(0)
        .with_message(message)
        .with_style(ProgressStyle::default_bar().template("{msg}").unwrap());
    let handler = |transit_process: fs_extra::TransitProcess| {
        update_progress_bar(
            &progress_bar,
            transit_process.total_bytes,
            transit_process.copied_bytes,
        );
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
pub fn replace_music(hm_mod: &HotlineMod, config: &Configs) {
    let music_path = hm_mod
        .music
        .as_ref()
        .expect("Should only be able to access this function when music is Some");
    let game_music_path = &config.paths_config.game_path.join(MUSIC_FILE_NAME);
    let copy_options = default_copy_options();
    let message = format_progress_bar_music_message(hm_mod);
    let progress_bar = ProgressBar::new(0)
        .with_message(message)
        .with_style(ProgressStyle::default_bar().template("{msg}").unwrap());
    let handler = |transit_process: TransitProcess| {
        update_progress_bar(
            &progress_bar,
            transit_process.total_bytes,
            transit_process.copied_bytes,
        );
    };
    let _ = copy_with_progress(music_path, game_music_path, &copy_options, handler)
        .expect("Could not copy the music file successfully.");
}

fn remove_mods_in_mods_dir(mods_path: &Path) {
    remove_dir_all(mods_path).expect("Error removing mods in the mods directory");
    create_dir(mods_path).expect("Error removing mods in the mods directory.");
}

fn format_progress_bar_music_message(hm_mod: &HotlineMod) -> String {
    format!("Copying {} music.", hm_mod.name.0)
}

fn format_progress_bar_mods_message(hm_mod: &HotlineMod) -> String {
    format!("Copying {} mods.", hm_mod.name.0)
}

fn default_copy_options() -> CopyOptions {
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    copy_options
}

fn update_progress_bar(progress_bar: &ProgressBar, total_bytes: u64, copied_bytes: u64) {
    let total = HumanBytes(total_bytes).to_string();
    let copied = HumanBytes(copied_bytes).to_string();

    progress_bar.set_message(format!("Copied: {} - Total: {}", copied, total));
}

const MUSIC_FILE_NAME: &str = "hlm2_music_desktop.wad";
