use inquire::InquireError;
use thiserror::Error;

use crate::{functions::prompt_user_select, hotline_mod::HotlineMod};

pub fn select_mod(mods: &[HotlineMod]) -> Result<HotlineMod, ChangeCurrentModError> {
    if mods.is_empty() {
        return Err(ChangeCurrentModError::EmptyMods);
    }

    match prompt_user_select("What mod do you wish to use?", mods.to_vec()) {
        Ok(desired_mod) => Ok(desired_mod),
        Err(InquireError::OperationCanceled) => Err(ChangeCurrentModError::UserCanceledOperation),
        Err(InquireError::OperationInterrupted) => {
            Err(ChangeCurrentModError::UserExitedApplication)
        }
        Err(err) => Err(From::from(err)),
    }
}

#[derive(Error, Debug)]
pub enum ChangeCurrentModError {
    #[error("You have no mods in your folder right now. Try downloading new mods or bringing your existing mods to this folder.")]
    EmptyMods,
    #[error("User pressed ESC when prompted.")]
    UserCanceledOperation,
    #[error("User wants to exit the application.")]
    UserExitedApplication,
    #[error("Inquire error")]
    InquireError(#[from] InquireError),
}
