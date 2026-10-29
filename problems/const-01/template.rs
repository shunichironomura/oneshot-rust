// The bodies are placeholders that return wrong values. Replace them.

pub const fn is_prime(n: u32) -> bool {
    let _ = n;
    false
}

pub const fn first_primes<const N: usize>() -> [u32; N] {
    [0; N]
}

pub const PRIMES_10: [u32; 10] = first_primes::<10>();
