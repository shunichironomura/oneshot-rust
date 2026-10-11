pub fn first_even(v: &[i32]) -> Option<i32> {
    let Some(x) = v.iter().find(|x| *x % 2 == 0) else {
        return None;
    };
    Some(*x)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    let mut given = false;
    if let Some(_) = title {
        given = true;
    }
    (given, title.unwrap_or(String::from("Untitled")))
}

pub fn first_middle_last(t: (i32, i32, i32, i32, i32)) -> (i32, i32, i32) {
    let (first, _, middle, _, last) = t;
    (first, middle, last)
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        if let Some(x) = slot {
            filled.push((i, *x));
        }
    }
    filled
}
