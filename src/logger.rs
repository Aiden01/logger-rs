use chrono::offset::Utc;
use std::default::Default;
use std::fmt;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

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

pub trait Bridge {
    type O;
    fn log(&self, imp: Importance, msg: &str) -> Self::O;
}

pub struct Logger<'a, T> {
    bridge: &'a dyn Bridge<O = T>,
}

impl<'a, T> Logger<'a, T> {
    pub fn new(bridge: &'a dyn Bridge<O = T>) -> Self {
        Logger { bridge }
    }

    pub fn fail<M: AsRef<str>>(&self, msg: M) -> T {
        self.bridge.log(Importance::Fail, msg.as_ref())
    }

    pub fn warn<M: AsRef<str>>(&self, msg: M) -> T {
        self.bridge.log(Importance::Warn, msg.as_ref())
    }

    pub fn debug<M: AsRef<str>>(&self, msg: M) -> T {
        self.bridge.log(Importance::Debug, msg.as_ref())
    }

    pub fn success<M: AsRef<str>>(&self, msg: M) -> T {
        self.bridge.log(Importance::Success, msg.as_ref())
    }
}

pub struct Console {
    date: bool,
}

impl Console {
    pub fn date(self, date: bool) -> Self {
        Console { date }
    }
}

impl Default for Console {
    fn default() -> Self {
        Console { date: false }
    }
}

impl Bridge for Console {
    type O = ();
    fn log(&self, imp: Importance, msg: &str) -> Self::O {
        if self.date {
            let today = Utc::today();
            println!("{}[{}]: {}", imp, today, msg);
        } else {
            println!("{}: {}", imp, msg);
        }
    }
}

pub struct File<'a> {
    out: &'a Path,
    append: bool,
}

impl<'a> File<'a> {
    pub fn new<T: Into<&'a Path>>(path: T) -> Self {
        File {
            out: path.into(),
            append: false,
        }
    }

    pub fn append(self, append: bool) -> Self {
        File { append, ..self }
    }

    fn format(&self, importance: Importance, msg: &str) -> String {
        let today = Utc::today();
        format!("{} [{}]: {}", importance, today, msg)
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

impl Bridge for File<'_> {
    type O = io::Result<()>;
    fn log(&self, imp: Importance, msg: &str) -> Self::O {
        let msg = self.format(imp, msg);
        self.write(&msg)
    }
}
