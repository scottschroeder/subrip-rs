#![deny(missing_docs)]
/*!
  A toolkit for parsing, authoring, and working with .srt files.
*/
use std::{fmt, time::Duration};

pub use parser::parse;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod parser;
pub mod utils;

#[derive(Debug)]
#[non_exhaustive]
/// Error type for crate
pub enum Error {
    /// Failure to parse SRT from provided text
    ParseError,
    /// Parsing completed with leftovers at the end of the file.
    ///
    /// Existing tools seem pretty forgiving with parsing SRT files,
    /// so you may occasionally find junk at the bottom of the file.
    ///
    /// This error includes whatever we could parse, as well as the index
    /// where parsing began to fail.
    ParseIncomplete(Vec<Subtitle>, usize),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
/// A single Subtitle record with timing and text information.
///
/// the [`std::fmt::Display`] impl for this type will reproduce
/// the SRT formatted text.
pub struct Subtitle {
    /// The index within the SRT file. The spec says this should be a number
    /// that increases through the file, but this is should not be relied upon.
    /// see [`utils::out_of_order_subs`] for more information.
    pub idx: u32,
    /// The timestamp where this subtitle should appear on screen.
    pub start: Duration,
    /// The timestamp where this subtitle should be removed from the screen.
    pub end: Duration,
    /// The contents of the subtitle text.
    ///
    /// This may contain some rudimetary formatting tags `<b>...</b>` which we
    /// currently make no effort to parse.
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
