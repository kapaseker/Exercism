use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters:HashSet<char> = HashSet::new(); 
    for x in candidate.to_lowercase().chars() {
        if x.is_ascii_alphabetic() {
            let letter = x.to_ascii_lowercase();
            if letters.contains(&letter) { 
                return false
            } 
            letters.insert(letter);
        }
    }
    true
}
