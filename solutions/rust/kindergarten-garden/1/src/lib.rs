pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut vec = vec![];
    let chars: Vec<char> = diagram.chars().into_iter().filter(|s| *s == 'G' || *s == 'C' || *s == 'R' || *s == 'V' ).collect();
    let gap = chars.len() / 2;
    let start = (student.as_bytes()[0] - 'A' as u8) as usize * 2;
    vec.push(match_flowers(&(chars[start])));
    vec.push(match_flowers(&(chars[start + 1])));
    vec.push(match_flowers(&(chars[start + gap])));
    vec.push(match_flowers(&(chars[start + gap + 1])));
    vec
}

fn match_flowers(a:&char) -> &'static str {
    match *a {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "",
    }
}
