use std::iter;

pub fn powers_of(base: u64) -> Vec<u64> {
    // `checked_mul` gives `None` on overflow, which ends the sequence.
    iter::successors(Some(1u64), |&p| p.checked_mul(base)).collect()
}
