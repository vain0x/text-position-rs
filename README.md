# Text Position

Provide text position representations and range of them.

## Install

*TODO: Publish to crates.io*

## Example

*TODO: Write example*

## Features flags

- `checked`: insert runtime checks for consistency of `CompositePosition`.

Usage:

```toml
text-position-rs = { ..., features = ["checked"] }
```

## See also

Related projects:

- [rust-analyzer/text-size](https://github.com/rust-analyzer/text-size)
    - `TextSize`: newtype of u32 as UTF-8 index
    - `TextRange`: range of `TextSize`
