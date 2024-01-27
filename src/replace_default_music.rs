use crate::configs::Configs;
use crate::hotline_mod::HotlineMod;
use crate::list_mods::list_mods;
use crate::replace_mod::replace_music;

pub fn replace_default_music(config: &Configs)  {
    let default_hm_mod = get_default_mod(config);

    match &default_hm_mod {
        Some(hm_mod) => match &hm_mod.music {
            Some(music) => replace_music(music, &hm_mod.name.0, config),
            None => on_none_mod(),
        },
        None => on_none_mod(),
    };
}

pub fn get_default_mod(config: &Configs) -> Option<HotlineMod> {
    list_mods(&config.paths_config.mods_group_path)
        .into_iter()
        .find(|hm_mod| hm_mod.name.directory_name() == "hotline_miami_2")
}

pub fn on_none_mod() {
    println!("Please provide a mod folder with the name hotline_miami_2 with the original music. For more information, visit the Github.");
}
