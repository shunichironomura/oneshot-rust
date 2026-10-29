pub fn quadrant(x: i32, y: i32) -> &'static str {
    match (x.signum(), y.signum()) {
        (1, 1) => "I",
        (-1, 1) => "II",
        (-1, -1) => "III",
        (1, -1) => "IV",
        (0, 0) => "origin",
        (0, _) => "y-axis",
        (_, 0) => "x-axis",
    }
}
