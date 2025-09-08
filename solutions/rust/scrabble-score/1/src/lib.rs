use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let protein_map = HashMap::from([
        ('D', 2u64),
        ('G', 2),
        ('B', 3),
        ('C', 3),
        ('M', 3),
        ('P', 3),
        ('F',4),
        ('H',4),
        ('V',4),
        ('W',4),
        ('Y',4),
        ('K',5),
        ('J',8),
        ('X',8),
        ('Q',10),
        ('Z',10),
    ]);

    word.chars().fold(0,|s,c| {
        s + if c.is_ascii()  { 
             *protein_map.get(&c.to_ascii_uppercase()).unwrap_or(&1) 
        }else {
            0u64 
        }
    })
}
