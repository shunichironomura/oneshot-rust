pub fn receipt_line(name: &str, qty: u32, price: f64) -> String {
    // On a string, the precision `.10` is a maximum length, so a longer name is cut.
    // On a float, `.2` is the number of digits after the point.
    format!("{name:<10.10} {qty:>3} x {price:>8.2}")
}
