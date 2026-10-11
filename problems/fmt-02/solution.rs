pub fn receipt_line(name: &str, qty: u32, price: f64) -> String {
    // On a string, the precision `.10` is a maximum length, so a longer name is cut.
    // On a float, `.2` is the number of digits after the point.
    format!("{name:<10.10} {qty:>3} x {price:>8.2}")
}

pub fn hex_color(r: u8, g: u8, b: u8) -> String {
    // `02` pads with zeros to two digits, and `X` is uppercase hexadecimal.
    format!("#{r:02X}{g:02X}{b:02X}")
}

pub fn bits(n: u8) -> String {
    // With `#`, the `0b` prefix counts toward the width: 2 + 8 = 10.
    format!("{n:#010b}")
}

pub fn table_row(cells: &[&str], width: usize) -> String {
    let mut row = String::from("|");
    for cell in cells {
        // `width$` reads the width from the variable `width`.
        // `^` centers, with the odd space on the right.
        row.push_str(&format!("{cell:^width$}|"));
    }
    row
}
