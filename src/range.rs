// LICENSE: CC0-1.0

use crate::{position::TextPosition, CompositePosition, Utf16Position, Utf8Index, Utf8Position};
use std::fmt::{self, Debug, Display, Formatter};

/// Range of text between two positions.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TextRange<P> {
    /// Start position of text range.
    pub start: P,

    /// End position of text range.
    pub end: P,
}

impl<P: TextPosition> TextRange<P> {
    pub const ZERO: Self = Self {
        start: P::ZERO,
        end: P::ZERO,
    };

    /// Create a range.
    pub fn new(start: P, end: P) -> Self {
        Self { start, end }
    }

    /// Create an empty range pointing to a position.
    pub fn empty(position: P) -> Self {
        Self {
            start: position.clone(),
            end: position,
        }
    }

    /// Create a range from origin to end.
    pub fn up_to(end: P) -> Self {
        Self {
            start: P::ZERO,
            end,
        }
    }

    /// Create an empty range pointing to the start position.
    pub fn start_point(self) -> Self {
        Self::empty(self.start)
    }

    /// Create an empty range pointing to the end position.
    pub fn end_point(self) -> Self {
        Self::empty(self.end)
    }

    /// Whether the range contains a position inclusively.
    /// True when `pos == self.end`.
    pub fn contains_inclusive(self, pos: P) -> bool {
        self.start <= pos && pos <= self.end
    }

    /// Whether the range contains another range entirely.
    pub fn covers(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    pub fn is_empty(self) -> bool {
        self.start == self.end
    }

    pub fn extend(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
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
        write!(f, "{}..{}", self.start, self.end)
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
        fmt_gnu(
            f,
            self.start.row,
            self.start.column,
            self.end.row,
            self.end.column,
        )
    }
}

impl Debug for TextRange<Utf16Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TextRange<Utf16Position> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_gnu(
            f,
            self.start.row,
            self.start.column,
            self.end.row,
            self.end.column,
        )
    }
}

impl Debug for TextRange<CompositePosition> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for TextRange<CompositePosition> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        fmt_gnu(
            f,
            self.start.row,
            self.start.column8,
            self.end.row,
            self.end.column8,
        )
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
        fn s(s: &str) -> Utf8Position {
            Utf8Position::from_str(s)
        }

        assert_eq!(
            format!("{}", TextRange::new(s("Hello, "), s("Hello, world"))),
            "1.8-1.13"
        );
    }
}
