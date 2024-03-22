// NeurOS - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::ansi::{BLUE, DEFAULT, GREEN, ORANGE, PURPLE, RED, YELLOW};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter, Result};
use spin::{Lazy, Mutex};

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    let logger = Logger::new();
    Mutex::new(logger)
});

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Debug, message);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Error, message);
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Fatal, message);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Info, message);
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Trace, message);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        let message = format!($($arg)*);
        LOGGER.lock().log(Level::Warn, message);
    };
}

pub enum Level {
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct Log {
    level: Level,
    message: String,
}

impl Display for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self.level {
            Level::Debug => format!("{GREEN}[DEBUG]{DEFAULT}"),
            Level::Error => format!("{RED}[ERROR]{DEFAULT}"),
            Level::Fatal => format!("{ORANGE}[FATAL]{DEFAULT}"),
            Level::Info => format!("{BLUE}[INFO]{DEFAULT}"),
            Level::Trace => format!("{PURPLE}[TRACE]{DEFAULT}"),
            Level::Warn => format!("{YELLOW}[WARN]{DEFAULT}"),
        };
        write!(f, "{} {}", label, self.message)?;
        Ok(())
    }
}

pub struct Logger {
    logs: Vec<Log>,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { logs: Vec::new() }
    }

    pub fn log(&mut self, level: Level, message: String) {
        let log = Log { level, message };
        self.logs.push(log);
    }

    pub fn get_logs(&self) -> &Vec<Log> {
        &self.logs
    }
}
