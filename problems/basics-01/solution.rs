const BASE: u64 = 10;

pub fn digital_root(mut n: u64) -> u64 {
    while n >= BASE {
        let mut sum = 0;
        let mut rest = n;
        while rest > 0 {
            sum += rest % BASE;
            rest /= BASE;
        }
        n = sum;
    }
    n
}
