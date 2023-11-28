use std::fs::DirEntry;

use crate::{functions::get_user_input, hotline_mod::HotlineMod, list_mods::format_mod_name};

pub fn get_desired_mod(all_mods: &[DirEntry]) -> HotlineMod {
    let all_mods_names = all_mods
        .iter()
        .map(DirEntry::file_name)
        .map(|path| format_mod_name(&path));

    for (i, name) in all_mods_names.enumerate() {
        println!("{i} - {name}");
    }

    let mut input = get_desired_index(all_mods);

    while input.is_none() {
        println!("Invalid input, please try again.");
        input = get_desired_index(all_mods);
    }

    let input = input.expect("is_none() check up there");
    let desired_mod = all_mods.get(input).expect("validation up there");
    HotlineMod::new(&desired_mod.path()).unwrap_or_else(|| {
        panic!(
            "Could not find the mod {} in the path {:?}",
            format_mod_name(&desired_mod.file_name()),
            desired_mod.path()
        )
    })
}

fn get_desired_index(all_mods: &[DirEntry]) -> Option<usize> {
    get_user_input(CHOOSE_YOUR_MOD)
        .parse::<usize>()
        .ok()
        .filter(|n| n >= &0 && n < &all_mods.len())
}

const CHOOSE_YOUR_MOD: &str =
    "What mod do you wish? (Use the number on the left to select the mod)";
