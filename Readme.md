# Emoji Converter

A Rust library to convert words in a sentence into emojis! This library provides a fun and simple way to add emoji representations to text.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
emoji_converter = "0.1.0"
```

### Usage

```rust
use emoji_converter::convert_to_emoji;

fn main() {
    let sentence = "Hello world! Love Pizza!";
    let emoji_sentence = convert_to_emoji(sentence);
    println!("{}", emoji_sentence);
}
```


### Summary

With documentation in place, our crate is ready for users to understand its purpose, how to use it, and what to expect from its functions. Next, we’ll move on to preparing `Cargo.toml` for publishing on crates.io.

