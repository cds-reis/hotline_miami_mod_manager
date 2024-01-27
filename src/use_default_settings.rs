use crate::{
    configs::Configs, functions::flush_configs, replace_default_music::{get_default_mod, on_none_mod}, replace_mod::replace_music
};

pub fn use_default_settings(configs: Configs) {
    let default_hm_mod = get_default_mod(&configs);
    if let Some(hm_mod) = default_hm_mod {
        match &hm_mod.music {
            Some(music) => replace_music(music, &hm_mod.name.0, &configs),
            None => on_none_mod(),
        }
        println!("Using {} now!", hm_mod.name);
        flush_configs(configs, hm_mod);
    }
}
