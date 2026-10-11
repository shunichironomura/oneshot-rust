pub trait Unit {
    fn name(&self) -> String;
    fn power(&self) -> u32;
    fn promoted(&self) -> Self;

    fn beats<U: Unit>(&self, other: &U) -> bool {
        self.power() > other.power()
    }
}

pub struct Knight {
    pub level: u32,
}

pub struct Archer {
    pub level: u32,
}

impl Unit for Knight {
    fn name(&self) -> String {
        format!("knight(lv{})", self.level)
    }

    fn power(&self) -> u32 {
        10 * self.level
    }

    fn promoted(&self) -> Knight {
        Knight {
            level: self.level + 1,
        }
    }
}

impl Unit for Archer {
    fn name(&self) -> String {
        format!("archer(lv{})", self.level)
    }

    fn power(&self) -> u32 {
        7 * self.level
    }

    fn promoted(&self) -> Archer {
        Archer {
            level: self.level + 1,
        }
    }
}

pub fn total_power(party: &[Box<dyn Unit>]) -> u32 {
    party.iter().map(|u| u.power()).sum()
}

pub fn strongest(party: &[Box<dyn Unit>]) -> Option<String> {
    let mut best: Option<&Box<dyn Unit>> = None;
    for unit in party {
        if best.is_none_or(|b| unit.power() > b.power()) {
            best = Some(unit);
        }
    }
    best.map(|u| u.name())
}
