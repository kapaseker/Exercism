/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    let mut s1 = s1.chars();
    let mut s2 = s2.chars();
    let mut distance = 0usize;

    loop {
        let c = s1.next();
        let b = s2.next();

        if c.is_none() {
            break;
        }

        if c.unwrap() != b.unwrap() {
            distance += 1;
        }
    }

    Some(distance)
}
