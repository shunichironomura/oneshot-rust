use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLock, RwLockWriteGuard, TryLockError};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

pub struct Store {
    settings: RwLock<Settings>,
    /// Only changed while the write lock on `settings` is held.
    version: AtomicU64,
}

impl Store {
    pub fn new(initial: Settings) -> Store {
        Store { settings: RwLock::new(initial), version: AtomicU64::new(1) }
    }

    pub fn read(&self) -> impl Deref<Target = Settings> + '_ {
        self.settings.read().unwrap()
    }

    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64 {
        let guard = self.settings.write().unwrap();
        self.apply(guard, f)
    }

    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
        match self.settings.try_write() {
            Ok(guard) => Some(self.apply(guard, f)),
            Err(TryLockError::WouldBlock) => None,
            Err(TryLockError::Poisoned(e)) => panic!("{e}"),
        }
    }

    pub fn version(&self) -> u64 {
        self.version.load(Ordering::Relaxed)
    }

    pub fn snapshot(&self) -> Settings {
        self.settings.read().unwrap().clone()
    }

    /// Runs `f` and bumps the version while the write lock is still held.
    fn apply(
        &self,
        mut settings: RwLockWriteGuard<'_, Settings>,
        f: impl FnOnce(&mut Settings),
    ) -> u64 {
        f(&mut settings);
        self.version.fetch_add(1, Ordering::Relaxed) + 1
    }
}
