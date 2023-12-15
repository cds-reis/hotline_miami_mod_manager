use inquire::Select;

use crate::hotline_mod::HotlineMod;

pub fn get_desired_mod(all_mods: Vec<HotlineMod>) -> HotlineMod {
    Select::new("What mod do you wish to use?", all_mods)
        .with_page_size(20)
        .prompt()
        .expect("Error while trying to read your input.")
}
