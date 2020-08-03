// LICENSE: CC0-1.0

mod position;
mod range;

pub use position::{
    composite_position::CompositePosition, utf16_position::Utf16Position, utf8_index::Utf8Index,
    utf8_position::Utf8Position, TextPosition,
};
pub use range::TextRange;
