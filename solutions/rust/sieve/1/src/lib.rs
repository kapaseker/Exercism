pub fn primes_up_to(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    
    if n == 0 { return primes; }

    let n = n + 1;

    let mut primes_check: Vec<bool> = vec![true; n as usize];
    let mut i = 2;
    while i < n {
        let iu = i as usize;
        if primes_check[iu] {
            primes.push(i);
            let iuu = i;
            if iuu * iuu < n {
                for j in (i * i..n).step_by(iu) {
                    primes_check[j as usize] = false
                }
            }
        }
        i += 1;
    }

    primes
}
