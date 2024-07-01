use std::{fmt::Display, sync::OnceLock};

use crate::LogLevel;


pub static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

#[inline]
pub fn println_log<S: Display>(log: S) {
    if *LOG_LEVEL.get_or_init(|| LogLevel::All) == LogLevel::All {
        println!("{}", log);
    }
}

#[inline]
pub fn println_output<S: Display>(log: S) {
    if *LOG_LEVEL.get_or_init(|| LogLevel::All) != LogLevel::None {
        println!("{}", log);
    }
}