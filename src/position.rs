// LICENSE: CC0-1.0

use std::ops::Add;

pub(crate) mod composite_position;
pub(crate) mod utf16_position;
pub(crate) mod utf8_index;
pub(crate) mod utf8_position;

/// Some representation of text position.
pub trait TextPosition: Clone + Ord + Add<Output = Self> {
    /// Origin.
    const ZERO: Self;

    /// Calculate a text position pointing to the end of string.
    fn from_str(s: &str) -> Self;

    /// Calculate the distance from `rhs` to `self`.
    ///
    /// Return `ZERO` if `self <= rhs`.
    fn saturating_sub(self, rhs: Self) -> Self;
}
