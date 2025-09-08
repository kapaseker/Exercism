use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> Luhn for T 
    where T : Display,
{
    fn valid_luhn(&self) -> bool {
        is_valid_luhn(&format!("{}", self))
    }
}

fn is_valid_luhn(code: &str) -> bool {
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
            sum += if num > 9 { num - 9 } else { num };
            char_index += 1;
        } else {
            return false;
        }
    }

    char_index > 1 && sum % 10 == 0
}
