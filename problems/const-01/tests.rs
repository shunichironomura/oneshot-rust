use crate::judge::{case, check, check_eq};
use crate::user::{PRIMES_10, first_primes, is_prime};

// Each case evaluates the functions in `const` items (at compile time) and calls them at runtime.
pub fn run() {
    case("example-1", || {
        const PRIME: [bool; 2] = [is_prime(97), is_prime(1)];
        check_eq(PRIME, [true, false])?;
        check_eq([is_prime(97), is_prime(1)], [true, false])
    });
    case("example-2", || {
        const P: [u32; 5] = first_primes::<5>();
        check_eq(P, [2, 3, 5, 7, 11])?;
        check_eq(PRIMES_10, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29])?;
        check_eq(first_primes::<5>(), [2, 3, 5, 7, 11])
    });
    case("case-1", || {
        // Small numbers, including squares of primes.
        check_eq([is_prime(0), is_prime(1), is_prime(2), is_prime(3)], [false, false, true, true])?;
        check_eq([is_prime(4), is_prime(9), is_prime(25), is_prime(49)], [false; 4])?;
        check_eq([is_prime(7919), is_prime(7917), is_prime(65537)], [true, false, true])?;
        check_eq((0..200).filter(|&n| is_prime(n)).count(), 46)
    });
    case("case-2", || {
        // Close to u32::MAX: 4_294_967_291 is the largest prime below 2^32.
        const BIG: [bool; 3] = [is_prime(u32::MAX), is_prime(u32::MAX - 1), is_prime(4_294_967_291)];
        check_eq(BIG, [false, false, true])
    });
    case("case-3", || {
        const NONE: [u32; 0] = first_primes::<0>();
        const ONE: [u32; 1] = first_primes::<1>();
        check_eq(NONE, [])?;
        check_eq(ONE, [2])?;
        check_eq(first_primes::<0>(), [])?;
        check_eq(first_primes::<3>(), [2, 3, 5])?;
        // The const argument is read from `PRIMES_10` at compile time.
        let five = first_primes::<{ PRIMES_10[2] as usize }>();
        check_eq(five.as_slice(), [2, 3, 5, 7, 11].as_slice())
    });
    case("case-4", || {
        const P: [u32; 100] = first_primes::<100>();
        check_eq((P[0], P[25], P[99]), (2, 101, 541))?;
        check_eq(P.iter().sum::<u32>(), 24133)?;
        check(P.windows(2).all(|w| w[0] < w[1]), "not ascending")
    });
}
