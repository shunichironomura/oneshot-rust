pub fn fizz_label(n: u32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

pub fn first_multiple_above(step: u64, limit: u64) -> u64 {
    let mut x = 0;
    loop {
        x += step;
        if x > limit {
            break x;
        }
    }
}
