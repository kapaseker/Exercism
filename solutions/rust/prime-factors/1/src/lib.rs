pub fn factors(n: u64) -> Vec<u64> {
    if n <= 1 { return vec![]; }

    let mut i = 2u64;
    let mut n = n;
    let mut factors = Vec::new();

    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        factors.push(n);
    }

    factors
}
