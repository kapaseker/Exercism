/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let valid_char = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'X'];
    let chars : Vec<char> = isbn.chars().filter(|s| *s != '-').collect();
    if chars.len() != 10 { return false }
    let mut sum = 0usize;
    for (index, char) in chars.iter().enumerate() {
        if let Some(v) = valid_char.iter().position(|r| *r == *char) {
            if v == 10 && index != 9 {
                return false
            }
            sum += (10 - index) * v;
        } else {
            return false
        }
    }

    sum % 11 == 0
}