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
