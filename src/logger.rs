use std::default::Default;
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::style::{DefaultStyle, Style};

/// Level of importance of the message
#[derive(Clone, Copy, PartialEq)]
pub enum Importance {
    Warn,
    Debug,
    Success,
    Fail,
}

impl fmt::Display for Importance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Importance::Warn => "Warn",
            Importance::Debug => "Debug",
            Importance::Success => "Success",
            Importance::Fail => "Error",
        };
        write!(f, "[{}]", s)
    }
}

/// The bridge receives a message and transfers it to a specific destination (e.g: console or a file).
/// You can make your own bridge by implementing this trait.
pub trait Bridge<T = ()> {
    fn log(&self, msg: &str) -> T;
}

/// Main Logger. Each logger has its own bridge where the messages are transferred.
pub struct Logger<T> {
    bridge: Box<dyn Bridge<T>>,
    style: Box<dyn Style>,
}

impl Default for Logger<()> {
    fn default() -> Self {
        Logger {
            style: Box::new(DefaultStyle::default()),
            bridge: Box::new(Console::default()),
        }
    }
}

impl<T> Logger<T> {
    pub fn style(self, style: Box<dyn Style>) -> Self {
        Logger { style, ..self }
    }

    pub fn bridge<U>(self, bridge: Box<dyn Bridge<U>>) -> Logger<U> {
        Logger {
            bridge,
            style: self.style,
        }
    }

    fn log(&self, imp: Importance, msg: &str) -> T {
        let msg = self.style.format(imp, msg);
        self.bridge.log(&msg)
    }

    pub fn fail<M: AsRef<str>>(&self, msg: M) -> T {
        self.log(Importance::Fail, msg.as_ref())
    }

    pub fn warn<M: AsRef<str>>(&self, msg: M) -> T {
        self.log(Importance::Warn, msg.as_ref())
    }

    pub fn debug<M: AsRef<str>>(&self, msg: M) -> T {
        self.log(Importance::Debug, msg.as_ref())
    }

    pub fn success<M: AsRef<str>>(&self, msg: M) -> T {
        self.log(Importance::Success, msg.as_ref())
    }
}

/// Bridge used to log onto the console.
pub struct Console;

impl Default for Console {
    fn default() -> Self {
        Console
    }
}

impl Bridge for Console {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

/// Bridge used to log inside a file.
/// There are two destination files, one for normal logs and the other for the errors.
/// The default destinations are both the same.
pub struct File<'a> {
    out: &'a Path,
    append: bool,
}

impl<'a> File<'a> {
    pub fn new<T: Into<&'a Path>>(path: T) -> Self {
        File {
            out: path.into(),
            append: true,
        }
    }

    /// Whether to overwrite the file completely or append the logs.
    pub fn append(self, append: bool) -> Self {
        File { append, ..self }
    }

    fn write(&self, content: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(self.append)
            .create(true)
            .write(true)
            .open(self.out)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Bridge<io::Result<()>> for File<'_> {
    fn log(&self, msg: &str) -> io::Result<()> {
        self.write(&msg)
    }
}
