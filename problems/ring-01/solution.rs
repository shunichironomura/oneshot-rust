pub struct Ring<T, const N: usize> {
    buf: [Option<T>; N],
    start: usize,
    len: usize,
}

impl<T, const N: usize> Ring<T, N> {
    pub const CAPACITY: usize = N;

    pub fn new() -> Self {
        Ring {
            // `[None; N]` would need `Option<T>: Copy`. A `const` block is evaluated anew for
            // every element instead of being copied.
            buf: [const { None }; N],
            start: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, x: T) -> Option<T> {
        if self.is_full() {
            // The slot of the oldest element becomes the slot of the newest one.
            let oldest = self.buf[self.start].replace(x);
            self.start = (self.start + 1) % N;
            oldest
        } else {
            self.buf[(self.start + self.len) % N] = Some(x);
            self.len += 1;
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let oldest = self.buf[self.start].take();
        self.start = (self.start + 1) % N;
        self.len -= 1;
        oldest
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        (0..self.len).map(|i| {
            self.buf[(self.start + i) % N]
                .as_ref()
                .expect("every slot in use holds an element")
        })
    }
}
