pub fn first_even(v: &[i32]) -> Option<i32> {
    let Some(x) = v.iter().find(|x| *x % 2 == 0);
    Some(*x)
}

pub fn title_or_default(title: Option<String>) -> (bool, String) {
    let mut given = false;
    if let Some(_t) = title {
        given = true;
    }
    (given, title.unwrap_or(String::from("Untitled")))
}

pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, Some(x)) in slots.iter().enumerate() {
        filled.push((i, *x));
    }
    filled
}
