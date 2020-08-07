// LICENSE: CC0-1.0

use crate::TextPosition;
use std::{
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, AddAssign},
};

/// Text position as (row, column) pair.
/// Column number (= length of the final line) is measured as number of UTF-16 code units (basically half of bytes).
/// Start from 0.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Utf16Position {
    pub row: u32,
    pub column: u32,
}

impl Utf16Position {
    pub const fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

impl TextPosition for Utf16Position {
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
            column: s[head..].encode_utf16().count() as u32,
        }
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        match self.row.cmp(&rhs.row) {
            Ordering::Less => Self::ZERO,
            Ordering::Equal => Self {
                row: 0,
                column: self.column.saturating_sub(rhs.column),
            },
            Ordering::Greater => Self {
                row: self.row - rhs.row,
                column: self.column,
            },
        }
    }
}

impl Add for Utf16Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if rhs.row == 0 {
            Self {
                row: self.row,
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

impl AddAssign for Utf16Position {
    fn add_assign(&mut self, rhs: Self) {
        let sum = *self + rhs;
        *self = sum;
    }
}

impl From<char> for Utf16Position {
    fn from(c: char) -> Self {
        if c == '\n' {
            Self { row: 1, column: 0 }
        } else {
            Self {
                row: 0,
                column: c.len_utf16() as u32,
            }
        }
    }
}

impl From<&'_ str> for Utf16Position {
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<Utf16Position> for (u32, u32) {
    fn from(pos: Utf16Position) -> (u32, u32) {
        (pos.row, pos.column)
    }
}

impl Debug for Utf16Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Utf16Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::{TextPosition, Utf16Position};

    const ZERO: Utf16Position = Utf16Position::ZERO;

    fn pos_at(row: u32, column: u32) -> Utf16Position {
        Utf16Position::new(row, column)
    }

    fn pos_of(s: &str) -> Utf16Position {
        Utf16Position::from_str(s)
    }

    #[test]
    fn test_from_str_empty() {
        assert_eq!(pos_of(""), ZERO);
    }

    #[test]
    fn test_from_str_ascii_single_line() {
        assert_eq!(pos_of("Hello, world!"), pos_at(0, 13));
    }

    #[test]
    fn test_from_str_ascii_multiple_line() {
        assert_eq!(pos_of("12345\n1234567\n12345"), pos_at(2, 5));
    }

    #[test]
    fn test_from_str_unicode() {
        assert_eq!(pos_of("いろはにほへと"), pos_at(0, 7));
    }

    #[test]
    fn test_from_str_surrogate_pair() {
        assert_eq!(pos_of("🐧"), pos_at(0, 2));
    }

    #[test]
    fn test_from_str_crlf() {
        assert_eq!(pos_of("\r\n"), pos_at(1, 0));
    }

    #[test]
    fn test_add_single_line() {
        assert_eq!(pos_of("12345") + pos_of("6789"), pos_at(0, 9))
    }

    #[test]
    fn test_add_newline() {
        assert_eq!(pos_of("12345") + pos_of("\n"), pos_at(1, 0));
    }

    #[test]
    fn test_add_multiple_line() {
        assert_eq!(pos_of("12345\n12345") + pos_of("67\n12345"), pos_at(2, 5))
    }

    #[test]
    fn test_saturating_sub_minus_row() {
        assert_eq!(
            pos_of("\n\n\n\n123456").saturating_sub(pos_of("\n\n\n\n\n1")),
            ZERO
        );
    }

    #[test]
    fn test_saturating_sub_minus_column() {
        assert_eq!(
            pos_of("\n\n\n\n123456").saturating_sub(pos_of("\n\n\n\n1234567")),
            ZERO
        );
    }

    #[test]
    fn test_saturating_sub_equal() {
        let pos = pos_of("\n\n\n\n123456");
        assert_eq!(pos.saturating_sub(pos), ZERO);
    }

    #[test]
    fn test_saturating_sub_plus_row() {
        assert_eq!(
            pos_of("\n\n\n12\n123456").saturating_sub(pos_of("\n\n\n12")),
            pos_of("\n123456")
        );
    }

    #[test]
    fn test_saturating_sub_plus_column() {
        assert_eq!(
            pos_of("\n\n\n\n123456").saturating_sub(pos_of("\n\n\n\n1")),
            pos_of("23456")
        );
    }

    #[test]
    fn test_saturating_sub_minus_row_in_number() {
        assert_eq!(pos_at(4, 6).saturating_sub(pos_at(5, 1)), ZERO);
    }

    #[test]
    fn test_saturating_sub_minus_column_in_number() {
        assert_eq!(pos_at(4, 6).saturating_sub(pos_at(4, 7)), ZERO);
    }

    #[test]
    fn test_saturating_sub_plus_row_in_number() {
        assert_eq!(pos_at(4, 6).saturating_sub(pos_at(3, 2)), pos_at(1, 6));
    }

    #[test]
    fn test_saturating_sub_plus_column_in_number() {
        assert_eq!(pos_at(4, 6).saturating_sub(pos_at(4, 1)), pos_at(0, 5));
    }

    #[test]
    fn test_display_zero() {
        assert_eq!(format!("{}", ZERO), "1:1");
    }

    #[test]
    fn test_display_nonzero() {
        assert_eq!(format!("{}", pos_at(3, 1)), "4:2");
    }
}
