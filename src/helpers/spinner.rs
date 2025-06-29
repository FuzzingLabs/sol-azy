use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

pub fn get_new_spinner(msg: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(msg);
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(50));
    spinner
}