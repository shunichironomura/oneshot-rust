use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub cents: i64,
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // `unsigned_abs` works for every value, including `i64::MIN`.
        let abs = self.cents.unsigned_abs();
        let digits = (abs / 100).to_string();

        let mut dollars = String::new();
        for (i, digit) in digits.chars().enumerate() {
            // A comma goes before every digit that starts a group of three, counted from the right.
            if i > 0 && (digits.len() - i) % 3 == 0 {
                dollars.push(',');
            }
            dollars.push(digit);
        }

        let sign = if self.cents < 0 { "-" } else { "" };
        let text = format!("{sign}${dollars}.{:02}", abs % 100);
        // `pad` applies the caller's width, fill and alignment, just like for a `&str`.
        f.pad(&text)
    }
}
