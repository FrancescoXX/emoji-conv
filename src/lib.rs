// src/lib.rs


//! # Emoji Converter
//!
//! A simple library to convert words in a sentence to emojis based on predefined mappings. 
//! This library is designed for fun applications and quick text transformations. 
//!
//! ## Example
//!
//! ```
//! use emoji_converter::convert_to_emojis;
//! 
//! let result = convert_to_emojis("I love coffee and pizza");
//! assert_eq!(result, "I ❤️ ☕️ and 🍕");
//! ``

mod emoji_mappings;

/// Converts words in a given sentence to their corresponding emojis.
///
/// # Arguments
/// * `text` - A string slice that holds the sentence to convert
///
/// # Returns
/// A new `String` with matching words replaced by emojis
///
/// # Examples
/// ```
/// let text = "I love coffee and pizza";
/// let result = emoji_converter::convert_to_emojis(text);
/// assert_eq!(result, "I ❤️ ☕️ and 🍕");
/// ```
pub fn convert_to_emojis(text: &str) -> String {
    let emoji_map = emoji_mappings::get_emoji_map();

    text.split_whitespace()
        .map(|word| {
            emoji_map.get(word).unwrap_or(&word).to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_emojis() {
        let input = "I love coffee and pizza";
        let expected = "I ❤️ ☕️ and 🍕";
        assert_eq!(convert_to_emojis(input), expected);
    }

    #[test]
    fn test_no_emojis() {
        let input = "No emojis here";
        let expected = "No emojis here";
        assert_eq!(convert_to_emojis(input), expected);
    }
}

