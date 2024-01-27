use crate::{
    functions::flush_configs, get_desired_mod, list_mods::list_mods, replace_mod::replace_mod,
    Configs,
};

pub fn change_mod(configs: Configs) {
    let all_mods = list_mods(&configs.paths_config.mods_group_path);
    if all_mods.is_empty() {
        println!("You have no mods in your folder right now. Try downloading new mods or bringing your existing mods to this folder.");
        return;
    }
    let desired_mod = get_desired_mod(all_mods);
    replace_mod(&desired_mod, &configs);
    println!("Using {} now!", desired_mod.name);
    flush_configs(configs, desired_mod);
}
