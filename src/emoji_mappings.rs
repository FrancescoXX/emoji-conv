// src/emoji_mappings.rs

use std::collections::HashMap;

/// Returns a HashMap of words to their corresponding emojis.
///
/// This mapping is used by the main emoji converter function to replace words
/// with emojis in a given sentence.
pub fn get_emoji_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("happy", "😊");
    map.insert("coffee", "☕️");
    map.insert("pizza", "🍕");
    map.insert("love", "❤️");
    map.insert("sun", "☀️");
    map
}
