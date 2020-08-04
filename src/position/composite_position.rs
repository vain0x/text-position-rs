// LICENSE: CC0-1.0

use crate::{TextPosition, Utf16Position, Utf8Index, Utf8Position};
use std::{
    cmp::Ordering,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign},
};

/// Text position represented by multiple measure:
///
/// - index: UTF-8 index
/// - row: Line number.
/// - column8: Column number as number of UTF-8 code units (bytes).
/// - column16: Column number as number of UTF-16 code units (basically half of bytes).
///
/// All of them start from 0.
#[derive(Copy, Clone, Debug)]
pub struct CompositePosition {
    /// UTF-8 index.
    pub index: u32,

    /// Line number.
    pub row: u32,

    /// Column number as number of UTF-8 code units (bytes).
    pub column8: u32,

    /// Column number as number of UTF-16 code units (basically half of bytes).
    pub column16: u32,
}

impl CompositePosition {
    pub const fn new(index: u32, row: u32, column8: u32, column16: u32) -> Self {
        Self {
            index,
            row,
            column8,
            column16,
        }
    }
}

impl TextPosition for CompositePosition {
    const ZERO: Self = Self {
        index: 0,
        row: 0,
        column8: 0,
        column16: 0,
    };

    fn from_str(s: &str) -> Self {
        let mut row = 0;
        let mut head = 0;

        while let Some(offset) = s[head..].find('\n') {
            row += 1;
            head += offset + 1;
        }

        Self {
            index: s.len() as u32,
            row: row as u32,
            column8: (s.len() - head) as u32,
            column16: s[head..].encode_utf16().count() as u32,
        }
    }

    fn saturating_sub(self, rhs: Self) -> Self {
        match self.row.cmp(&rhs.row) {
            Ordering::Less => Self::ZERO,
            Ordering::Equal => Self {
                index: self.index.saturating_sub(rhs.index),
                row: 0,
                column8: self.column8.saturating_sub(rhs.column8),
                column16: self.column16.saturating_sub(rhs.column8),
            },
            Ordering::Greater => Self {
                index: self.index.saturating_sub(rhs.index),
                row: self.row - rhs.row,
                column8: self.column8,
                column16: self.column16,
            },
        }
    }
}

impl From<char> for CompositePosition {
    fn from(c: char) -> Self {
        if c == '\n' {
            Self {
                index: 1,
                row: 1,
                column8: 0,
                column16: 0,
            }
        } else {
            let index = c.len_utf8() as u32;
            Self {
                index,
                row: 0,
                column8: index,
                column16: c.len_utf16() as u32,
            }
        }
    }
}

impl AddAssign for CompositePosition {
    fn add_assign(&mut self, rhs: Self) {
        let sum = *self + rhs;
        *self = sum;
    }
}

impl Add for CompositePosition {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let index = self.index + rhs.index;

        if self.row == 0 {
            Self {
                index,
                row: rhs.row,
                column8: self.column8 + rhs.column8,
                column16: self.column16 + rhs.column16,
            }
        } else {
            Self {
                index,
                row: self.row + rhs.row,
                column8: rhs.column8,
                column16: rhs.column16,
            }
        }
    }
}

impl From<CompositePosition> for Utf8Index {
    fn from(pos: CompositePosition) -> Self {
        Utf8Index::new(pos.index)
    }
}

impl From<CompositePosition> for Utf8Position {
    fn from(pos: CompositePosition) -> Self {
        Utf8Position::new(pos.row, pos.column8)
    }
}

impl From<CompositePosition> for Utf16Position {
    fn from(pos: CompositePosition) -> Self {
        Utf16Position::new(pos.row, pos.column16)
    }
}

#[allow(unused)]
fn assert_equality_consistency(it: &CompositePosition, other: &CompositePosition, equal: bool) {
    if equal {
        assert_eq!(Utf8Position::from(*it), Utf8Position::from(*other));
        assert_eq!(Utf16Position::from(*it), Utf16Position::from(*other));
    } else {
        assert_ne!(Utf8Position::from(*it), Utf8Position::from(*other));
        assert_ne!(Utf16Position::from(*it), Utf16Position::from(*other));
    }
}

#[allow(unused)]
fn assert_ordering_consistency(
    it: &CompositePosition,
    other: &CompositePosition,
    ordering: Option<Ordering>,
) {
    assert_eq!(
        Utf8Position::from(*it).partial_cmp(&Utf8Position::from(*other)),
        ordering
    );
    assert_eq!(
        Utf16Position::from(*it).partial_cmp(&Utf16Position::from(*other)),
        ordering
    );
}

impl PartialEq for CompositePosition {
    fn eq(&self, other: &Self) -> bool {
        let equal = self.index == other.index;

        #[cfg(feature = "checked")]
        assert_equality_consistency(self, other, equal);

        equal
    }
}

impl Eq for CompositePosition {}

impl PartialOrd for CompositePosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = self.index.partial_cmp(&other.index);

        #[cfg(feature = "checked")]
        assert_ordering_consistency(self, other, ordering);

        ordering
    }
}

impl Ord for CompositePosition {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.index.cmp(&other.index);

        #[cfg(feature = "checked")]
        assert_ordering_consistency(self, other, Some(ordering));

        ordering
    }
}

impl Display for CompositePosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.row + 1, self.column8 + 1)
    }
}

impl Hash for CompositePosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::{position::TextPosition, CompositePosition};

    const ZERO: CompositePosition = CompositePosition::ZERO;

    fn pos_of(s: &str) -> CompositePosition {
        CompositePosition::from_str(s)
    }

    #[test]
    fn test_from_str_empty() {
        assert_eq!(pos_of(""), CompositePosition::ZERO);
    }

    #[test]
    fn test_from_str_ascii_single_line() {
        assert_eq!(
            pos_of("Hello, world!"),
            CompositePosition::new(13, 0, 13, 13)
        );
    }

    #[test]
    fn test_from_str_ascii_multiple_line() {
        assert_eq!(
            pos_of("12345\n1234567\n12345"),
            CompositePosition::new(19, 2, 5, 5)
        );
    }

    #[test]
    fn test_from_str_unicode() {
        assert_eq!(
            pos_of("いろはにほへと\nちりぬるを\nわかよたれそ\nつねならむ"),
            CompositePosition::new(72, 3, 15, 15)
        );
    }

    #[test]
    fn test_from_str_surrogate_pair() {
        assert_eq!(pos_of("🐧"), CompositePosition::new(4, 0, 4, 2));
    }

    #[test]
    fn test_from_str_crlf() {
        assert_eq!(pos_of("\r\n"), CompositePosition::new(2, 1, 0, 0));
    }

    #[test]
    fn test_add_single_line() {
        assert_eq!(
            pos_of("12345") + pos_of("6789"),
            CompositePosition::new(9, 0, 9, 9)
        )
    }

    #[test]
    fn test_add_multiple_line() {
        assert_eq!(
            pos_of("12345\n12345") + pos_of("67\n12345"),
            CompositePosition::new(19, 2, 5, 5)
        )
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
    fn test_display_zero() {
        assert_eq!(format!("{}", ZERO), "1:1");
    }

    #[test]
    fn test_display_nonzero() {
        assert_eq!(format!("{}", pos_of("\n\n\nxx")), "4:3");
    }
}
