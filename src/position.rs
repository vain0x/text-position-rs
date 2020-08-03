// LICENSE: CC0-1.0

pub(crate) mod composite_position;
pub(crate) mod utf16_position;
pub(crate) mod utf8_index;
pub(crate) mod utf8_position;

/// Some representation of text position.
pub trait TextPosition: Clone + Ord {
    /// Origin.
    const ZERO: Self;

    /// Calculate a text position pointing to the end of string.
    fn from_str(s: &str) -> Self;
}
