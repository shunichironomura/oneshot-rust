pub fn to_levels(samples: &[f32]) -> Vec<u8> {
    samples
        .iter()
        .map(|&s| {
            // SAFETY: samples are always in 0.0..=1.0, so the level always fits in a u8.
            unsafe { (s * 255.0).round().to_int_unchecked::<u8>() }
        })
        .collect()
}
