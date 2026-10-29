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
