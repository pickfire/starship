use ansi_term::Color;
use chrono::{DateTime, Local};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time")?;

    let mut time_format = module.config_value_str("format").unwrap_or("%H:%M").to_owned();

    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);
    if is_12hr == true {
        time_format = module.config_value_str("format").unwrap_or("%I:%M %p").to_owned();
    }

    let local: DateTime<Local> = Local::now();

    module.set_style(Color::Yellow.bold());
    module.new_segment("time", &format!("[{}]", local.format(&time_format)));
    module.get_prefix().set_value("");

    Some(module)
}