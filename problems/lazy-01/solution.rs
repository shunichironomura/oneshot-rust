pub struct Lazy<T, F: FnOnce() -> T> {
    // `init` is `Some` until it is called. `value` is `Some` from then on.
    init: Option<F>,
    value: Option<T>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub fn new(init: F) -> Self {
        Lazy { init: Some(init), value: None }
    }

    pub fn get(&mut self) -> &T {
        self.value.get_or_insert_with(|| {
            let init = self.init.take().expect("an earlier call to init panicked");
            init()
        })
    }

    pub fn is_ready(&self) -> bool {
        self.value.is_some()
    }

    pub fn into_value(self) -> T {
        match self.value {
            Some(value) => value,
            None => {
                let init = self.init.expect("an earlier call to init panicked");
                init()
            }
        }
    }
}
