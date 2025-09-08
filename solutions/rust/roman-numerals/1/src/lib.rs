use std::fmt::{Display, Formatter, Result};

pub struct Roman(u32);

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        let mut str = String::new();
        let mut num = self.0;

        loop {
            if num == 0 {
                break
            }
            if num >= 1000 {
                str.push('M');
                num -= 1000;
            } else if num >= 900 {
                str.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                str.push('D');
                num -= 500;
            } else if num >= 400 {
                str.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                str.push('C');
                num -= 100;
            } else if num >= 90 {
                str.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                str.push('L');
                num -= 50;
            } else if num >= 40 {
                str.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                str.push('X');
                num -= 10;
            } else if num >= 9 {
                str.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                str.push('V');
                num -= 5;
            } else if num >= 4 {
                str.push_str("IV");
                num -= 4;
            } else {
                str.push('I');
                num -= 1;
            }
        }

        write!(_f, "{}", str)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self(num)
    }
}
