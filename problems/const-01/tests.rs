use crate::judge::{case, check, check_eq};
use crate::user::{PRIMES_10, first_primes, gcd, is_prime, weighted_sum};

// Each case evaluates the functions in `const` items (at compile time) and calls them at runtime.
pub fn run() {
    case("sample-1", || {
        const G: u64 = gcd(84, 36);
        const PRIME: [bool; 2] = [is_prime(97), is_prime(1)];
        check_eq(G, 12)?;
        check_eq(PRIME, [true, false])?;
        check_eq(gcd(84, 36), 12)?;
        check_eq([is_prime(97), is_prime(1)], [true, false])
    });
    case("sample-2", || {
        const P: [u32; 5] = first_primes::<5>();
        check_eq(P, [2, 3, 5, 7, 11])?;
        check_eq(PRIMES_10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])?;
        check_eq(first_primes::<5>(), [2, 3, 5, 7, 11])
    });
    case("sample-3", || {
        const W: u32 = weighted_sum(b"ab");
        check_eq(W, 293)?;
        check_eq(weighted_sum(b"ab"), 293)
    });
    case("hidden-1", || {
        // A zero argument in either position, and the order of the arguments.
        const ZERO: [u64; 3] = [gcd(0, 0), gcd(0, 7), gcd(u64::MAX, 0)];
        check_eq(ZERO, [0, 7, u64::MAX])?;
        check_eq([gcd(0, 0), gcd(7, 0), gcd(0, u64::MAX)], [0, 7, u64::MAX])?;
        check_eq([gcd(17, 5), gcd(48, 180), gcd(180, 48)], [1, 12, 12])
    });
    case("hidden-2", || {
        // Huge arguments: a subtraction-based loop would never finish.
        const HUGE: u64 = gcd(u64::MAX - 1, 4);
        check_eq(HUGE, 2)?;
        check_eq(gcd(u64::MAX, 2), 1)?;
        check_eq(gcd(1 << 63, 3 << 61), 1 << 61)
    });
    case("hidden-3", || {
        // Small numbers, including squares of primes.
        check_eq([is_prime(0), is_prime(1), is_prime(2), is_prime(3)], [false, false, true, true])?;
        check_eq([is_prime(4), is_prime(9), is_prime(25), is_prime(49)], [false; 4])?;
        check_eq([is_prime(7919), is_prime(7917), is_prime(65537)], [true, false, true])?;
        check_eq((0..200).filter(|&n| is_prime(n)).count(), 46)
    });
    case("hidden-4", || {
        // Close to u32::MAX: 4_294_967_291 is the largest prime below 2^32.
        const BIG: [bool; 3] = [is_prime(u32::MAX), is_prime(u32::MAX - 1), is_prime(4_294_967_291)];
        check_eq(BIG, [false, false, true])
    });
    case("hidden-5", || {
        const NONE: [u32; 0] = first_primes::<0>();
        const ONE: [u32; 1] = first_primes::<1>();
        check_eq(NONE, [])?;
        check_eq(ONE, [2])?;
        check_eq(first_primes::<0>(), [])?;
        check_eq(first_primes::<3>(), [2, 3, 5])?;
        // The const argument is computed by `gcd` at compile time.
        let six = first_primes::<{ gcd(12, 18) as usize }>();
        check_eq(six.as_slice(), [2, 3, 5, 7, 11, 13].as_slice())
    });
    case("hidden-6", || {
        const P: [u32; 100] = first_primes::<100>();
        check_eq((P[0], P[25], P[99]), (2, 101, 541))?;
        check_eq(P.iter().sum::<u32>(), 24133)?;
        check(P.windows(2).all(|w| w[0] < w[1]), "not ascending")
    });
    case("hidden-7", || {
        // Weights go past 255, and they start at 1 for every slice, including subslices.
        const EMPTY: u32 = weighted_sum(b"");
        const FULL: u32 = weighted_sum(&[255; 300]);
        check_eq(EMPTY, 0)?;
        check_eq(FULL, 11_513_250)?;
        check_eq(weighted_sum(&[]), 0)?;
        check_eq(weighted_sum(b"abc"), 590)?;
        check_eq(weighted_sum(&[0, 0, 1]), 3)?;
        check_eq(weighted_sum(&b"xab"[1..]), 293)
    });
}
