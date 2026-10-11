const BASE = 10;

pub fn digital_root(n: u64) -> u64 {
    while n >= BASE {
        let mut sum = 0;
        let mut rest = n;
        while rest > 0 {
            sum += rest % BASE;
            rest /= BASE;
        }
        n = sum;
    }
    n;
}

pub fn fizz_label(n: u32) -> String {
    let label = if n % 15 == 0 {
        "FizzBuzz"
    } else if n % 3 == 0 {
        "Fizz"
    } else if n % 5 == 0 {
        "Buzz"
    } else {
        n.to_string()
    };
    label
}

pub fn first_multiple_above(step: u64, limit: u64) -> u64 {
    let mut x = 0;
    let found = loop {
        x += step;
        if x > limit {
            break;
        }
    };
    found
}
