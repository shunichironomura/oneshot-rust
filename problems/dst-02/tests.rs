use crate::judge::{case, check, check_eq};
use crate::user::{Packet, checksum, make};

pub fn run() {
    case("example-1", || {
        let a = make(1, [1, 2, 3]);
        check_eq(checksum(&a), 7)?;
        check_eq(checksum(&Packet { id: 7, payload: [5u8, 6] }), 18)
    });
    case("example-2", || {
        let ps = vec![make(1, [1, 2, 3]), make(2, [9; 5]), make(3, [])];
        let sums: Vec<u32> = ps.iter().map(|p| checksum(p)).collect();
        check_eq(sums, [7, 47, 3])
    });
    case("case-1", || {
        // The fields of packets made by `make`, including an empty payload.
        let a = make(42, [7, 8]);
        check_eq(a.id, 42)?;
        check_eq(&a.payload, [7, 8])?;
        let e = make(5, []);
        check(e.payload.is_empty(), "the payload should be empty")?;
        check_eq(checksum(&e), 5)
    });
    case("case-2", || {
        // Large bytes, and more packets that the harness creates itself.
        check_eq(checksum(&make(1000, [255; 4])), 2020)?;
        check_eq(checksum(&Packet { id: 3, payload: [] }), 3)?;
        let local = Packet { id: 1, payload: [10, 20, 30, 40] };
        check_eq(checksum(&local), 101)
    });
    case("case-3", || {
        // A `Vec` filled one packet at a time, and bytes changed through the box.
        let mut ps = Vec::new();
        ps.push(make(1, [1]));
        ps.push(make(2, [1, 2, 3]));
        ps.push(make(3, [1, 2, 3, 4, 5, 6, 7, 8]));
        ps.push(make(4, []));
        ps[0].payload[0] = 200;
        let sums: Vec<u32> = ps.iter().map(|p| checksum(p)).collect();
        check_eq(sums, [201, 8, 39, 4])
    });
}
