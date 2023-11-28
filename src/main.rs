use configs::Configs;
use get_desired_mod::get_desired_mod;
use list_mods::list_mods;

use crate::replace_mod::replace_mod;

pub mod configs;
pub mod functions;
pub mod get_desired_mod;
pub mod hotline_mod;
pub mod list_mods;
pub mod replace_mod;

fn main() {
    let configs = Configs::new();
    let all_mods = list_mods(&configs.mods_group_path);
    if all_mods.is_empty() {
        println!("You have no mods in this folder right now. Try downloading new mods or bringing your existing mods to this folder.");
        return;
    }
    let desired_mod = get_desired_mod(&all_mods);
    replace_mod(&desired_mod, &configs);
    println!("Using {} now!", desired_mod.formatted_name())
}
