use std::process::{exit, Command};

pub fn run_hotline_miami_2() -> ! {
    let output = build_command().output();
    match output {
        Ok(_) => exit(0),
        Err(err) => {
            eprintln!("Something went wrong while opening the game.");
            eprintln!("Error: {:?}", err);
            exit(123)
        }
    }
}

fn build_command() -> Command {
    let mut command = Command::new("cmd");
    command.args(["/C", "start", "", HOTLINE_MIAMI_2_STEAM_URL]);
    command
}

const HOTLINE_MIAMI_2_STEAM_URL: &str = r"steam://rungameid/274170";
