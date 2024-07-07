use crate::{
    configs::Configs, functions::flush_configs, replace_default_music::{get_default_mod, on_none_mod}, replace_mod::{replace_mods, replace_music}
};

pub fn use_default_settings(configs: &mut Configs) {
    if let Some(hm_mod) = get_default_mod(&configs) {
        match &hm_mod.music {
            Some(music) => replace_music(music, &hm_mod.name, &configs),
            None => on_none_mod(),
        }
        replace_mods(&hm_mod, &configs);
        println!("Using {} now!", hm_mod.name);
        configs.set_current_mod(current_mod)
    }
}
