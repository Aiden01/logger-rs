use crate::logger::Importance;
use chrono::offset::Utc;
use colored::{ColoredString, Colorize};

/// Trait used to style the logs
pub trait Style {
    fn format(&self, imp: Importance, msg: &str) -> String;
}

fn with_color(imp: &Importance, msg: &str) -> ColoredString {
    match imp {
        Importance::Fail => msg.red(),
        Importance::Warn => msg.yellow(),
        Importance::Debug => msg.blue(),
        Importance::Success => msg.green(),
    }
}

/// Default style used for the logs
/// E.g:
/// [Error]: This is an error message
pub struct DefaultStyle {
    date: bool,
    colored: bool,
}

impl DefaultStyle {
    pub fn date(self, date: bool) -> Self {
        DefaultStyle { date, ..self }
    }

    pub fn colored(self, colored: bool) -> Self {
        DefaultStyle { colored, ..self }
    }
}

impl Default for DefaultStyle {
    fn default() -> Self {
        DefaultStyle {
            date: false,
            colored: true,
        }
    }
}

impl Style for DefaultStyle {
    fn format(&self, imp: Importance, msg: &str) -> String {
        let log = if self.date {
            let today = Utc::today();
            format!("{} [{}]: {}", imp, today, msg)
        } else {
            format!("{}: {}", imp, msg)
        };

        if self.colored {
            with_color(&imp, &log).to_string()
        } else {
            log
        }
    }
}

/// Simple and minimalist style.
/// E.g:
/// ▶ This is an error messsage.
pub struct Arrow {
    colored: bool,
}

impl Default for Arrow {
    fn default() -> Self {
        Arrow { colored: true }
    }
}

impl Arrow {
    pub fn colored(self, colored: bool) -> Self {
        Arrow { colored }
    }
}

impl Style for Arrow {
    fn format(&self, imp: Importance, msg: &str) -> String {
        let log = format!("▶ {}", msg);
        if self.colored {
            with_color(&imp, &log).to_string()
        } else {
            log
        }
    }
}
