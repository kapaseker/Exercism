use std::fmt::Display;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        Self::is_valid_luhn(&self.0)
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
}

impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(input: T) -> Self {
        Self(format!("{}", input))
    }
}
