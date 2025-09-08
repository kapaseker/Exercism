use std::collections::HashMap;
use regex::Regex;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    Regex::new(r"(\w+'\w+)|(\w+)").unwrap().find_iter(words).map(|s| s.as_str().to_lowercase()).fold(HashMap::new(), |mut map, value| {
        *map.entry(value).or_default() += 1;
        map
    })
}
