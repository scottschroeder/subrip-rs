use std::{fmt, time::Duration};

pub use parser::parse;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod parser;
pub mod utils;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    ParseError,
    ParseIncomplete(Vec<Subtitle>, usize),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Subtitle {
    pub idx: u32,
    pub start: Duration,
    pub end: Duration,
    pub text: String,
}

fn fmt_duration_srt(d: Duration, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let millis = d.as_millis() as u64;
    let secs = millis / 1000;
    let millis = millis % 1000;
    let minutes = secs / 60;
    let secs = secs % 60;
    let hours = minutes / 60;
    let minutes = minutes % 60;
    write!(f, "{}:{}:{},{}", hours, minutes, secs, millis)
}

impl fmt::Display for Subtitle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.idx)?;
        fmt_duration_srt(self.start, f)?;
        write!(f, " --> ")?;
        fmt_duration_srt(self.end, f)?;
        writeln!(f)?;
        writeln!(f, "{}", self.text)?;
        writeln!(f)
    }
}
