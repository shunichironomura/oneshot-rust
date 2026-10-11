use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

pub struct Store {
    // Choose the fields yourself.
    _todo: (),
}

impl Store {
    pub fn new(initial: Settings) -> Store {
        todo!()
    }

    pub fn read(&self) -> impl Deref<Target = Settings> + '_ {
        // A placeholder so that the template compiles. Replace it.
        todo!() as Box<Settings>
    }

    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64 {
        todo!()
    }

    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
        todo!()
    }

    pub fn version(&self) -> u64 {
        todo!()
    }

    pub fn snapshot(&self) -> Settings {
        todo!()
    }
}
