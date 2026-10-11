// The bodies are placeholders that return wrong values. Replace them.

pub const fn gcd(a: u64, b: u64) -> u64 {
    let _ = (a, b);
    0
}

pub const fn is_prime(n: u32) -> bool {
    let _ = n;
    false
}

pub const fn first_primes<const N: usize>() -> [u32; N] {
    [0; N]
}

pub const fn weighted_sum(bytes: &[u8]) -> u32 {
    let _ = bytes;
    0
}

pub const PRIMES_10: [u32; 10] = first_primes::<10>();
