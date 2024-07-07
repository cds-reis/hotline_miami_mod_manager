use crate::configs::paths_config::GamePath;
use crate::configs::Configs;
use crate::hotline_mod::HotlineMod;
use crate::manager::DefaultHotlineMod;
use crate::replace_mod::replace_music;

pub fn replace_default_music(default_mod: &DefaultHotlineMod, game_path: &GamePath) {
    replace_music(music_path, mod_name, game_path);
}

pub fn get_default_mod(config: &Configs) -> Option<HotlineMod> {
    list_mods(&config.paths_config.mods_group_path)
        .into_iter()
        .find(|hm_mod| hm_mod.name.directory_name() == "hotline_miami_2")
}
