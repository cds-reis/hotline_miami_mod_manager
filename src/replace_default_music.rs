use crate::configs::Configs;
use crate::list_mods::list_mods;
use crate::replace_mod::replace_music;

pub fn replace_default_music(config: &Configs) {
    let mods = list_mods(&config.paths_config.mods_group_path);
    let default_hm_mod = mods
        .iter()
        .find(|hm_mod| hm_mod.name.directory_name() == "hotline_miami_2");

    match default_hm_mod {
        Some(music) => replace_music(music, config),
        None => on_none_mod(),
    }
}

fn on_none_mod() {
    println!("Please provide a mod folder with the name hotline_miami_2 with the original music. For more information, visit the Github.");
}
