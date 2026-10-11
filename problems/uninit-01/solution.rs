pub fn build<T>(order: &[usize], mut f: impl FnMut(usize) -> T) -> Vec<T> {
    // Start with an empty slot for each index. `vec![None; n]` would need `T: Clone`.
    let mut slots: Vec<Option<T>> = order.iter().map(|_| None).collect();
    for &i in order {
        // If `f` panics, `slots` is dropped while unwinding, and it drops exactly the values
        // produced so far.
        slots[i] = Some(f(i));
    }
    // `order` is a permutation, so every slot is filled now.
    slots.into_iter().map(|slot| slot.unwrap()).collect()
}
