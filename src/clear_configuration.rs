use std::{fs, io};

use crate::configs::{PATH_CONFIGS_FILE_NAME, MODS_CONFIG_FILE_NAME};

pub fn clear_configuration() {
    if let Err(error) = remove_files() {
        println!("Error clearing the configs. {error}");
    } else {
        println!("Successfully cleared your configuration!");
    };
}

fn remove_files() -> io::Result<()> {
    fs::remove_file(PATH_CONFIGS_FILE_NAME)?;
    fs::remove_file(MODS_CONFIG_FILE_NAME)?;
    Ok(())
}
