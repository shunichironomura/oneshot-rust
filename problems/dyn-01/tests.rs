use crate::judge::{case, check, check_eq};
use crate::user::{Archer, Knight, Unit, strongest, total_power};

pub fn run() {
    case("example-1", || {
        let party: Vec<Box<dyn Unit>> =
            vec![Box::new(Knight { level: 2 }), Box::new(Archer { level: 3 })];
        check_eq(total_power(&party), 41)?;
        check_eq(strongest(&party), Some("archer(lv3)".to_string()))
    });
    case("example-2", || {
        let k = Knight { level: 1 }.promoted();
        check_eq(k.name(), "knight(lv2)".to_string())?;
        check(k.beats(&Archer { level: 2 }), "20 beats 14")
    });
    case("case-1", || {
        let party: Vec<Box<dyn Unit>> = Vec::new();
        check_eq(total_power(&party), 0)?;
        check_eq(strongest(&party), None)
    });
    case("case-2", || {
        let a = Archer { level: 10 };
        check(!a.beats(&Knight { level: 7 }), "70 does not beat 70")?;
        check(a.promoted().beats(&Knight { level: 7 }), "77 beats 70")?;
        check(
            !Knight { level: 0 }.beats(&Archer { level: 0 }),
            "0 does not beat 0",
        )
    });
    case("case-3", || {
        let party: Vec<Box<dyn Unit>> = vec![
            Box::new(Archer { level: 10 }),
            Box::new(Knight { level: 6 }.promoted()),
            Box::new(Archer { level: 1 }.promoted()),
        ];
        check_eq(total_power(&party), 70 + 70 + 14)?;
        check_eq(strongest(&party), Some("archer(lv10)".to_string()))
    });
    case("case-4", || {
        let party: Vec<Box<dyn Unit>> = vec![Box::new(Knight { level: 5 })];
        check_eq(party[0].name(), "knight(lv5)".to_string())?;
        check_eq(strongest(&party), Some("knight(lv5)".to_string()))
    });
}
