pub const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub const fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let mut d = 3;
    // `d <= n / d` means `d * d <= n`, but cannot overflow near `u32::MAX`.
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

pub const fn first_primes<const N: usize>() -> [u32; N] {
    let mut primes = [0; N];
    let mut count = 0;
    let mut candidate = 2;
    while count < N {
        if is_prime(candidate) {
            primes[count] = candidate;
            count += 1;
        }
        candidate += 1;
    }
    primes
}

pub const fn weighted_sum(bytes: &[u8]) -> u32 {
    let mut sum = 0;
    let mut i = 0;
    while i < bytes.len() {
        sum += (i as u32 + 1) * bytes[i] as u32;
        i += 1;
    }
    sum
}

pub const PRIMES_10: [u32; 10] = first_primes::<10>();
