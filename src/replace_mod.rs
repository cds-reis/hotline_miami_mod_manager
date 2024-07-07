use std::{
    fs::{read_dir, DirEntry},
    path::Path,
};

use fs_extra::{
    copy_items_with_progress,
    file::{copy_with_progress, CopyOptions, TransitProcess},
};
use fs_extra::{
    dir::{self, TransitProcessResult},
    file::remove,
};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};

use crate::{
    configs::{
        paths_config::{GamePath, ModsPath, ProgramPath},
        Configs,
    },
    hotline_mod::{HotlineMod, HotlineModName},
    replace_default_music::replace_default_music,
};

pub fn replace_mod(hm_mod: &HotlineMod, config: &Configs) {
    match &hm_mod.music {
        Some(music) => {
            replace_music(music, &hm_mod.name, config);
        }
        None => {
            replace_default_music(config);
        }
    };
    replace_mods(hm_mod, config);
}

pub fn replace_mods(hm_mod: &HotlineMod, config: &Configs) {
    let game_mods_path = config.paths_config().mods_path();
    remove_mods_in_mods_dir(game_mods_path);
    if hm_mod.mods.is_empty() {
        return;
    }
    let message = format_progress_bar_mods_message(&hm_mod.name);
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
        game_mods_path.path(),
        &dir::CopyOptions::new(),
        handler,
    )
    .expect("Could not copy your mods successfully.");
}
pub fn replace_music(music_path: &Path, mod_name: &HotlineModName, game_path: &GamePath) {
    let game_music_path = game_path
        .path()
        .join(MUSIC_FILE_NAME);
    let copy_options = default_copy_options();
    let message = format_progress_bar_music_message(mod_name);
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

fn remove_mods_in_mods_dir(mods_path: &ModsPath) {
    let files_to_remove = read_dir(mods_path.path())
        .expect("Could not read the mods directory")
        .filter_map(|file| file.ok())
        .filter(file_is_patchwad)
        .collect::<Vec<_>>();

    for file in files_to_remove {
        _ = remove(file.path());
    }
}

fn file_is_patchwad(dir_entry: &DirEntry) -> bool {
    let path = dir_entry.path();

    let is_file = path.is_file();
    let is_patchwad = path
        .extension()
        .map(|extension| extension.eq_ignore_ascii_case("patchwad"))
        .unwrap_or(false);

    is_file && is_patchwad
}

fn format_progress_bar_music_message(mod_name: &HotlineModName) -> String {
    format!("Copying {} music.", mod_name)
}

fn format_progress_bar_mods_message(mod_name: &HotlineModName) -> String {
    format!("Copying {} mods.", mod_name)
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
