pub fn square_of_sum(n: u32) -> u32 {
    return ((n + 1) * n / 2).pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut n = n;
    let mut sum = 0u32;
    while n != 1 {
        sum += n.pow(2);
        n -= 1;
    }
    sum + 1
}

pub fn difference(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        for j in (i + 1)..=n {
            sum += i * j * 2;
        }
    }
    sum
}
