use crate::judge::{case, check_eq};
use crate::user::{Packet, describe, label};

pub fn run() {
    case("example-1", || {
        let ls = vec![label(1, 5), label(2, "x")];
        check_eq(describe(&ls), ["#1: 5", "#2: x"])
    });
    case("case-1", || {
        // Payloads of many types in one `Vec`, and an empty part of it.
        let owned = String::from("héllo");
        let ls = vec![
            label(1, 1.5),
            label(2, 'z'),
            label(3, owned),
            label(4, -7),
            label(5, true),
            label(6, ""),
        ];
        check_eq(describe(&ls), ["#1: 1.5", "#2: z", "#3: héllo", "#4: -7", "#5: true", "#6: "])?;
        check_eq(describe(&ls[..0]), Vec::<String>::new())
    });
    case("case-2", || {
        // The fields of a packet made by `label`, and a packet that the harness boxes itself.
        let p = label(9, 12u8);
        check_eq(p.id, 9)?;
        check_eq(p.payload.to_string(), "12")?;
        let mixed = vec![p, Box::new(Packet { id: 3, payload: 'q' })];
        check_eq(describe(&mixed), ["#9: 12", "#3: q"])?;
        check_eq(describe(&mixed[1..]), ["#3: q"])
    });
}
