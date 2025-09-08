use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn modular_pow(base: u64, exponent: u64, modular: u64) -> u64 {
    
    if modular == 1 { return 0; }
    
    let modular = modular as u128;
    let mut base = base as u128;
    let mut result = 1u128;
    let mut exponent = exponent as u128;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modular;
        }
        exponent /= 2;
        base = base * base % modular;
    }

    result as u64
}