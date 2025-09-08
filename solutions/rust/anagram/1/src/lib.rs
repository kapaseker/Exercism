use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut ch_map = HashMap::new();
    
    word.to_lowercase().chars().for_each(|c| {
        ch_map.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
    });

    possible_anagrams.into_iter().filter(|it| {
        let mut char_map = ch_map.clone();
        if it.to_lowercase() != word.to_lowercase() {
            for x in it.to_lowercase().chars() {
                let entry = char_map.get_mut(&x);
                match entry {
                    Some(count) => {
                        *count -= 1;
                        if *count == 0 { char_map.remove(&x); }
                    }
                    None => {
                        return false
                    }
                }
            }
            return char_map.is_empty();
        } else {
            false
        }
    }).map(|s| *s).collect::<HashSet<&'a str>>()
}