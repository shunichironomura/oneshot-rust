pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    // A float-to-integer `as` cast saturates: a level below 0 becomes 0, a level above 255
    // becomes 255, and NaN becomes 0.
    samples.iter().map(|&s| (s * 255.0).round() as u8).collect()
}
