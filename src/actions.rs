use std::fmt::Display;

use inquire::Select;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    ChangeMod,
    ChangeConfigurationPath,
}

impl Action {
    const VARIANTS: &'static [Action] = &[Action::ChangeMod, Action::ChangeConfigurationPath];
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::ChangeMod => write!(f, "Change the current mod."),
            Action::ChangeConfigurationPath => write!(f, "Change one of your paths."),
        }
    }
}

pub fn get_desired_action() -> Action {
    Select::new("What do you want to do?", Action::VARIANTS.to_vec())
        .prompt()
        .expect("Error trying to read you input.")
}
