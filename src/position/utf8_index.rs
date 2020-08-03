// LICENSE: CC0-1.0

use crate::TextPosition;
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign},
};

/// Text position represented by UTF-8 index.
/// Start from 0.
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Utf8Index {
    /// Index to UTF-8 string.
    pub index: u32,
}

impl Utf8Index {
    pub const fn new(index: u32) -> Self {
        Self { index }
    }
}

impl TextPosition for Utf8Index {
    const ZERO: Self = Self { index: 0 };

    fn from_str(s: &str) -> Self {
        Self::new(s.len() as u32)
    }
}

impl AddAssign for Utf8Index {
    fn add_assign(&mut self, rhs: Self) {
        self.index += rhs.index;
    }
}

impl Add for Utf8Index {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.index + rhs.index)
    }
}

impl From<char> for Utf8Index {
    fn from(c: char) -> Self {
        Self::new(c.len_utf8() as u32)
    }
}

impl From<&'_ str> for Utf8Index {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl Debug for Utf8Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Utf8Index {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.index, f)
    }
}
