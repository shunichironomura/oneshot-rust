pub struct Validator<'a> {
    rules: Vec<Box<dyn Fn(&str) -> bool + 'a>>,
}

impl<'a> Validator<'a> {
    pub fn new() -> Self {
        Validator { rules: Vec::new() }
    }

    pub fn add(&mut self, rule: impl Fn(&str) -> bool + 'a) {
        self.rules.push(Box::new(rule));
    }

    pub fn add_boxed(&mut self, rule: Box<dyn Fn(&str) -> bool + 'a>) {
        self.rules.push(rule);
    }

    pub fn check(&self, s: &str) -> bool {
        self.rules.iter().all(|rule| rule(s))
    }

    pub fn passing<'s>(&self, items: &[&'s str]) -> Vec<&'s str> {
        items.iter().copied().filter(|s| self.check(s)).collect()
    }
}

pub fn banning(words: &[String]) -> Validator<'_> {
    let mut v = Validator::new();
    v.add(move |s| !words.iter().any(|w| s.contains(w.as_str())));
    v
}
