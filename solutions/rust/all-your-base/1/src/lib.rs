#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    
    if from_base < 2 { 
        return Err(Error::InvalidInputBase);
    }
    
    if to_base < 2 { 
        return Err(Error::InvalidOutputBase);
    }
    
    let mut sum = 0u32;
    let mut len = number.len() as u32;
    for x in number {
        if *x < from_base {
            sum += x * from_base.pow(len - 1);
            len -= 1;
        } else { 
            return Err(Error::InvalidDigit(*x));
        }
    }
    
    let mut digits = vec![];

    loop {
        digits.push(sum % to_base);
        sum /= to_base;
        
        if sum == 0 { 
            break;
        }
    }
    
    digits.reverse();
    
    Ok(digits) 
}
