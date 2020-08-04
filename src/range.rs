// LICENSE: CC0-1.0

use crate::{position::TextPosition, CompositePosition, Utf16Position, Utf8Index, Utf8Position};
use std::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Add, Range},
};

// DESIGN: Prefer (index, len) over (start, end)
//         because invariant `len >= 0` is easier to encode in type system
//         than `start <= end`.

/// Range of text between two positions.
///
/// ```
/// use text_position_rs::{Utf8Position, TextRange};
///
/// // Example of creation.
/// let start = Utf8Position::new(2, 4);
/// let end = Utf8Position::new(4, 8);
/// let range: TextRange<Utf8Position> =
///     TextRange::from(start..end);
///
/// // Example of operation.
/// let middle = Utf8Position::new(3, 3);
/// assert!(range.contains_inclusive(middle));
/// ```
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TextRange<P> {
    /// Start position of text range.
    pub index: P,

    /// Size of text range.
    pub len: P,
}

impl<P: TextPosition> TextRange<P> {
    pub const ZERO: Self = Self {
        index: P::ZERO,
        len: P::ZERO,
    };

    /// Create a range.
    pub fn at(index: P, len: P) -> Self {
        Self { index, len }
    }

    /// Create an empty range pointing to a position.
    pub fn empty(index: P) -> Self {
        Self {
            index,
            len: P::ZERO,
        }
    }

    /// Create a range from origin to end.
    pub fn up_to(len: P) -> Self {
        Self {
            index: P::ZERO,
            len,
        }
    }

    pub fn start(self) -> P {
        self.index
    }

    pub fn end(self) -> P {
        self.index + self.len
    }

    /// Empty range pointing to the start position.
    pub fn to_start(self) -> Self {
        Self::empty(self.end())
    }

    /// Empty range pointing to the end position.
    pub fn to_end(self) -> Self {
        Self::empty(self.end())
    }

    /// Whether the range contains a position inclusively.
    ///
    /// True if `pos == self.end()`.
    pub fn contains_inclusive(self, pos: P) -> bool {
        self.index <= pos && pos <= self.end()
    }

    /// Whether the range contains another range entirely.
    pub fn covers(self, other: Self) -> bool {
        // QUESTION: More efficient way?
        self.clone().start() <= other.clone().start() && other.end() <= self.end()
    }

    /// Whether the range is empty.
    ///
    /// ```
    /// use text_position_rs::{TextRange, Utf8Index};
    ///
    /// // Empty:
    /// let empty: TextRange<Utf8Index> = TextRange::ZERO;
    /// assert!(empty.is_empty());
    /// assert!(TextRange::empty(Utf8Index::new(1)).is_empty());
    ///
    /// // Non-empty:
    /// assert!(!TextRange::from(Utf8Index::new(1)..Utf8Index::new(2)).is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        self.len == P::ZERO
    }

    /// Extend a range to cover the other range.
    ///
    /// ```
    /// use text_position_rs::{TextRange, Utf8Index};
    ///
    /// let first_range = TextRange::from(Utf8Index::new(2)..Utf8Index::new(4));
    /// let second_range = TextRange::from(Utf8Index::new(6)..Utf8Index::new(8));
    /// let extended_range = TextRange::from(Utf8Index::new(2)..Utf8Index::new(8));
    /// assert_eq!(first_range.extend(second_range), extended_range);
    ///
    /// // Reversed case.
    /// assert_eq!(second_range.extend(first_range), extended_range);
    /// ```
    pub fn extend(self, other: Self) -> Self {
        // QUESTION: More efficient way?
        let start = self.clone().start().min(other.clone().start());
        let end = self.end().max(other.end());
        Self::from(start..end)
    }
}

impl<P: TextPosition> From<Range<P>> for TextRange<P> {
    fn from(range: Range<P>) -> Self {
        let Range { start, end } = range;
        Self {
            index: start.clone(),
            len: end.saturating_sub(start),
        }
    }
}

impl<P: TextPosition + Add<Output = P>> From<TextRange<P>> for Range<P> {
    fn from(range: TextRange<P>) -> Self {
        let TextRange { index, len } = range;
        Range {
            start: index.clone(),
            end: index + len,
        }
    }
}

impl Debug for TextRange<Utf8Index> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TextRange<Utf8Index> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}..{}", self.start(), self.end())
    }
}

impl Debug for TextRange<Utf8Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

/// <https://www.gnu.org/prep/standards/html_node/Errors.html>
fn fmt_gnu(
    f: &mut Formatter,
    start_row: u32,
    start_column: u32,
    end_row: u32,
    end_column: u32,
) -> fmt::Result {
    write!(
        f,
        "{}.{}-{}.{}",
        start_row + 1,
        start_column + 1,
        end_row + 1,
        end_column + 1
    )
}

impl Display for TextRange<Utf8Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start = self.start();
        let end = self.end();
        fmt_gnu(f, start.row, start.column, end.row, end.column)
    }
}

impl Debug for TextRange<Utf16Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TextRange<Utf16Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start = self.start();
        let end = self.end();
        fmt_gnu(f, start.row, start.column, end.row, end.column)
    }
}

impl Debug for TextRange<CompositePosition> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TextRange<CompositePosition> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let start = self.start();
        let end = self.end();
        fmt_gnu(f, start.row, start.column8, end.row, end.column8)
    }
}

#[cfg(test)]
mod tests {
    use crate::{TextPosition, TextRange, Utf8Position};

    #[test]
    fn test_display_zero() {
        assert_eq!(format!("{}", TextRange::<Utf8Position>::ZERO), "1.1-1.1");
    }

    #[test]
    fn test_display_nonzero() {
        fn pos_of(s: &str) -> Utf8Position {
            Utf8Position::from_str(s)
        }

        assert_eq!(
            format!(
                "{}",
                TextRange::from(pos_of("Hello, ")..pos_of("Hello, world"))
            ),
            "1.8-1.13"
        );
    }
}
