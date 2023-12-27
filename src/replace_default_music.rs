use std::process;
use std::thread::sleep;
use std::time::Duration;

use crate::configs::Configs;
use crate::list_mods::list_mods;
use crate::replace_mod::replace_music;

pub fn replace_default_music(config: &Configs) {
    let mods = list_mods(&config.paths_config.mods_group_path);
    let music = mods
        .iter()
        .find(|hm_mod| hm_mod.name.directory_name() == "hotline_miami_2")
        .and_then(|default| default.music.as_ref());

    match music {
        Some(music) => replace_music(music, config),
        None => on_none_music(),
    }
}

fn on_none_music() {
    println!("Please provide a mod folder with the name hotline_miami_2 with the original music. For more information, visit the Github.");
    sleep(Duration::from_secs(4));
    process::exit(0);
}
