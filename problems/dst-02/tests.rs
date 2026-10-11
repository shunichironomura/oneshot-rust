use crate::judge::{case, check, check_eq};
use crate::user::{Packet, checksum, describe, label, longest, make};

pub fn run() {
    case("sample-1", || {
        let a = make(1, [1, 2, 3]);
        check_eq(checksum(&a), 7)?;
        check_eq(checksum(&Packet { id: 7, payload: [5u8, 6] }), 18)
    });
    case("sample-2", || {
        let ps = vec![make(1, [1, 2, 3]), make(2, [9; 5]), make(3, [])];
        check_eq(longest(&ps), Some(2))
    });
    case("sample-3", || {
        let ls = vec![label(1, 5), label(2, "x")];
        check_eq(describe(&ls), ["#1: 5", "#2: x"])
    });
    case("hidden-1", || {
        // The fields of packets made by `make`, including an empty payload.
        let a = make(42, [7, 8]);
        check_eq(a.id, 42)?;
        check_eq(&a.payload, [7, 8])?;
        let e = make(5, []);
        check(e.payload.is_empty(), "the payload should be empty")?;
        check_eq(checksum(&e), 5)
    });
    case("hidden-2", || {
        // Large bytes, and more packets that the harness creates itself.
        check_eq(checksum(&make(1000, [255; 4])), 2020)?;
        check_eq(checksum(&Packet { id: 3, payload: [] }), 3)?;
        let local = Packet { id: 1, payload: [10, 20, 30, 40] };
        check_eq(checksum(&local), 101)
    });
    case("hidden-3", || {
        // On a tie, the first packet wins. Parts of the `Vec` can be passed too.
        let ps = vec![make(4, [1, 2]), make(8, [3, 4]), make(6, [0])];
        check_eq(longest(&ps), Some(4))?;
        check_eq(longest(&ps[1..]), Some(8))?;
        check_eq(longest(&ps[2..]), Some(6))?;
        check_eq(longest(&ps[..0]), None)?;
        let empty = vec![make(1, []), make(2, [])];
        check_eq(longest(&empty), Some(1))
    });
    case("hidden-4", || {
        // A `Vec` filled one packet at a time, and bytes changed through the box.
        let mut ps = Vec::new();
        ps.push(make(1, [1]));
        ps.push(make(2, [1, 2, 3]));
        ps.push(make(3, [1, 2, 3, 4, 5, 6, 7, 8]));
        ps.push(make(4, []));
        ps[0].payload[0] = 200;
        let sums: Vec<u32> = ps.iter().map(|p| checksum(p)).collect();
        check_eq(sums, [201, 8, 39, 4])?;
        check_eq(longest(&ps), Some(3))
    });
    case("hidden-5", || {
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
    case("hidden-6", || {
        // The fields of a packet made by `label`, and a packet that the harness boxes itself.
        let p = label(9, 12u8);
        check_eq(p.id, 9)?;
        check_eq(p.payload.to_string(), "12")?;
        let mixed = vec![p, Box::new(Packet { id: 3, payload: 'q' })];
        check_eq(describe(&mixed[1..]), ["#3: q"])
    });
}
