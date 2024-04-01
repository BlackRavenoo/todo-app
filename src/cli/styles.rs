use colored::{ColoredString, Colorize};

use crate::config::TextSettings;

pub fn use_style(text: String, config: &TextSettings) -> ColoredString {
    let mut text = text.color(config.color);
    if config.bold {
        text = text.bold();
    }
    if config.italic {
        text = text.italic()
    }
    text
}