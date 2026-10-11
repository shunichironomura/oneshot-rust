pub struct Collatz {
    next: Option<u64>,
}

impl Collatz {
    pub fn new(start: u64) -> Self {
        Collatz {
            next: if start == 0 { None } else { Some(start) },
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.next?;
        self.next = match n {
            1 => None,
            n if n % 2 == 0 => Some(n / 2),
            n => Some(3 * n + 1),
        };
        Some(n)
    }
}
