use std::{io, process::Command};

use thiserror::Error;

pub fn run_hotline_miami_2() -> Result<(), RunHotlineMiamiError> {
    let output = build_command().output();
    match output {
        Ok(_) => Ok(()),
        Err(err) => Err(RunHotlineMiamiError::from(err)),
    }
}

#[derive(Error, Debug)]
pub enum RunHotlineMiamiError {
    #[error("Something went wrong while opening the game. Error: {0}")]
    IoError(#[from] io::Error),
}

fn build_command() -> Command {
    let mut command = Command::new("cmd");
    command.args(["/C", "start", "", HOTLINE_MIAMI_2_STEAM_URL]);
    command
}

const HOTLINE_MIAMI_2_STEAM_URL: &str = r"steam://rungameid/274170";
