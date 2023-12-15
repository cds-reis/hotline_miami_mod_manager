use std::fs::DirEntry;

use inquire::Select;

use crate::{hotline_mod::HotlineMod, list_mods::format_mod_name};

pub fn get_desired_mod(all_mods: &[DirEntry]) -> HotlineMod {
    let desired_mod = get_mod(all_mods);
    HotlineMod::new(&desired_mod.path()).unwrap_or_else(|| {
        panic!(
            "Could not find the mod {} in the path {:?}",
            format_mod_name(&desired_mod.file_name()),
            desired_mod.path()
        )
    })
}

fn get_mod(mods: &[DirEntry]) -> &DirEntry {
    let all_mods_names = mods
        .iter()
        .map(DirEntry::file_name)
        .map(|path| format_mod_name(&path))
        .collect::<Vec<String>>();

    let desired_mod_name = Select::new("What mod do you wish to use?", all_mods_names)
        .with_page_size(20)
        .prompt()
        .expect("Error while trying to read your input.");

    return mods
        .iter()
        .find(|hm_mod| format_mod_name(&hm_mod.file_name()) == desired_mod_name)
        .expect("Should have the mod");
}

