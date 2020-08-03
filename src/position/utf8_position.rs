// LICENSE: CC0-1.0

use crate::TextPosition;
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign},
};

/// Text position as (row, column) pair.
/// Column number (= length of the final line) is measured as number of UTF-8 code units (bytes).
/// Start from 0.
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Utf8Position {
    pub row: u32,
    pub column: u32,
}

impl Utf8Position {
    pub const fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

impl TextPosition for Utf8Position {
    const ZERO: Self = Self { row: 0, column: 0 };

    fn from_str(s: &str) -> Self {
        let mut row = 0;
        let mut head = 0;

        while let Some(offset) = s[head..].find('\n') {
            row += 1;
            head += offset + 1;
        }

        Self {
            row: row as u32,
            column: (s.len() - head) as u32,
        }
    }
}

impl Add for Utf8Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.row == 0 {
            Self {
                row: rhs.row,
                column: self.column + rhs.column,
            }
        } else {
            Self {
                row: self.row + rhs.row,
                column: rhs.column,
            }
        }
    }
}

impl AddAssign for Utf8Position {
    fn add_assign(&mut self, rhs: Self) {
        let sum = *self + rhs;
        *self = sum;
    }
}

impl From<char> for Utf8Position {
    fn from(c: char) -> Self {
        if c == '\n' {
            Self { row: 1, column: 0 }
        } else {
            Self {
                row: 0,
                column: c.len_utf8() as u32,
            }
        }
    }
}

impl From<&'_ str> for Utf8Position {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<Utf8Position> for (u32, u32) {
    fn from(pos: Utf8Position) -> (u32, u32) {
        (pos.row, pos.column)
    }
}

impl Debug for Utf8Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Utf8Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Utf8Position;
    use crate::position::TextPosition;

    #[test]
    fn test_from_str_empty() {
        assert_eq!(Utf8Position::from_str(""), Utf8Position::ZERO);
    }

    #[test]
    fn test_from_str_ascii_single_line() {
        assert_eq!(
            Utf8Position::from_str("Hello, world!"),
            Utf8Position::new(0, 13)
        );
    }

    #[test]
    fn test_from_str_ascii_multiple_line() {
        assert_eq!(
            Utf8Position::from_str("12345\n1234567\n12345"),
            Utf8Position::new(2, 5)
        );
    }

    #[test]
    fn test_from_str_unicode() {
        assert_eq!(Utf8Position::from_str("🐧"), Utf8Position::new(0, 4));
    }

    #[test]
    fn test_from_str_crlf() {
        assert_eq!(Utf8Position::from_str("\r\n"), Utf8Position::new(1, 0));
    }

    #[test]
    fn test_add_single_line() {
        assert_eq!(
            Utf8Position::from_str("12345") + Utf8Position::from_str("6789"),
            Utf8Position::new(0, 9)
        )
    }

    #[test]
    fn test_add_multiple_line() {
        assert_eq!(
            Utf8Position::from_str("12345\n12345") + Utf8Position::from_str("67\n12345"),
            Utf8Position::new(2, 5)
        )
    }

    #[test]
    fn test_display_zero() {
        assert_eq!(format!("{}", Utf8Position::ZERO), "1:1");
    }

    #[test]
    fn test_display_nonzero() {
        assert_eq!(format!("{}", Utf8Position::new(3, 1)), "4:2");
    }
}
