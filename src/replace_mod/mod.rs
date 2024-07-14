pub mod replace_mods;
pub mod replace_music;

use indicatif::{HumanBytes, ProgressBar};

fn update_progress_bar(progress_bar: &ProgressBar, total_bytes: u64, copied_bytes: u64) {
    let total = HumanBytes(total_bytes).to_string();
    let copied = HumanBytes(copied_bytes).to_string();

    progress_bar.set_message(format!("Copied: {copied} - Total: {total}"));
}
