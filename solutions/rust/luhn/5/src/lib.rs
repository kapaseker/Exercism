pub fn is_valid(code: &str) -> bool {
    let mut char_index = 0;
    let mut sum = 0u32;
    for x in code.chars().rev() {
        if x == ' ' {
            continue;
        }

        if !x.is_ascii_digit() {
            return false;
        }

        let num = x.to_digit(10);

        if let Some(num) = num {
            let num = if char_index % 2 == 1 {
                num * 2
            } else {
                num
            };
            sum += (if num > 9 { num - 9 } else { num }) as u32;
            char_index += 1;
        } else {
            return false;
        }
    }

    char_index > 1 && sum % 10 == 0
}