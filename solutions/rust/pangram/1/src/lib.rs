/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut char_arr = [0usize;26];
        
    sentence.chars().for_each(|c| {
       if c.is_ascii_alphabetic() { 
           let index = (c.to_ascii_lowercase() as u8 - 'a' as u8) as usize;
           char_arr[index] = char_arr[index] + 1; 
       } 
    });
    
    char_arr.iter().all(|s| *s > 0)
}
