pub fn is_armstrong_number(num: u32) -> bool {
    let mut count = 0;
    
    let mut digit = num;
    
    while digit != 0 {
        digit /= 10;
        count += 1;
    }
    
    let mut sum:u64 = 0;
    
    digit = num;
    
    while digit != 0 {
        sum = sum.wrapping_add(u64::from((digit % 10).wrapping_pow(count)));
        digit /= 10;
    }
    
    u64::from(num) == sum
}
