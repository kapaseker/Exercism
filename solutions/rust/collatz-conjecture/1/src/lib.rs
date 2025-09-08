pub fn collatz(n: u64) -> Option<u64> {

    if n == 0 {
        return None;
    }

    let mut n = n;
    let mut step = 0;
    while n != 1 {
        if n & 1 == 1 {
          n = n * 3 + 1;  
        } else {
            n = n >> 1;
        }
        step += 1;
    }
    
    Some(step)
}
