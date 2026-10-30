use crate::judge::{case, check_eq};
use crate::user::Light;

pub fn run() {
    case("example-1", || {
        check_eq(Light::Green.next(), Light::Yellow)?;
        check_eq(Light::Red.duration_secs(), 30)?;
        check_eq(Light::Yellow.time_until_green(), 35)
    });
    case("example-2", || {
        check_eq(Light::Flashing(2).next(), Light::Flashing(1))?;
        check_eq(Light::Flashing(0).next(), Light::Yellow)
    });
    case("case-1", || {
        check_eq(Light::Yellow.next(), Light::Red)?;
        check_eq(Light::Red.next(), Light::Green)
    });
    case("case-2", || {
        check_eq(Light::Yellow.duration_secs(), 5)?;
        check_eq(Light::Green.duration_secs(), 25)?;
        check_eq(Light::Flashing(0).duration_secs(), 1)?;
        check_eq(Light::Flashing(7).duration_secs(), 1)
    });
    case("case-3", || {
        check_eq(Light::Green.time_until_green(), 0)?;
        check_eq(Light::Red.time_until_green(), 30)
    });
    case("case-4", || {
        check_eq(Light::Flashing(0).time_until_green(), 36)?;
        check_eq(Light::Flashing(2).time_until_green(), 38)
    });
}
