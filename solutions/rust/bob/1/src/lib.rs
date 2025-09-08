pub fn reply(message: &str) -> &str {
    let mut chars = message.chars();

    if chars.all(|s| s.is_whitespace()) {
        return "Fine. Be that way!";
    }

    let all_up = chars.any(|s| s.is_alphabetic())
        &&
        chars.all(|s| {
            if !s.is_alphabetic() {
                true
            } else {
                s.is_uppercase()
            }
        });


    let end_question_mark = matches!(message.trim_end().chars().last() ,Some('?'));

    if message.is_empty() {}

    if all_up {
        return if end_question_mark {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        };
    } else if end_question_mark {
        return "Sure.";
    }

    return "Whatever.";
}
