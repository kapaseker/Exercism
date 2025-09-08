pub fn encode(source: &str) -> String {
    let mut chars = source.chars();
    let mut count = 1;
    let mut str = String::new();

    let mut ch = match chars.next() {
        Some(c) => c,
        None => return str,
    };

    for c in chars {
        if c == ch {
            count += 1;
        } else {
            if count > 1 {
                str.push_str(&count.to_string());
            }
            str.push(ch);
            ch = c;
            count = 1;
        }
    }

    if count > 1 {
        str.push_str(&count.to_string());
    }
    str.push(ch);

    str
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count_str = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            count_str.push(c);
        } else {
            let count = if count_str.is_empty() {
                1
            } else {
                count_str.parse().unwrap_or(1)
            };

            for _ in 0..count {
                result.push(c);
            }

            count_str.clear();
        }
    }

    result
}
