use std::cell::RefCell;
use std::mem::ManuallyDrop;
use std::ptr;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct TempFile {
    name: String,
    data: Vec<u8>,
    log: Log,
}

impl TempFile {
    pub fn create(name: &str, log: &Log) -> TempFile {
        log.borrow_mut().push(format!("create {name}"));
        TempFile { name: name.to_string(), data: Vec::new(), log: Rc::clone(log) }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn rename(mut self, new_name: &str) -> TempFile {
        self.log.borrow_mut().push(format!("rename {} -> {new_name}", self.name));
        // Change the name in place and return `self` as a whole: nothing is moved out of it.
        self.name = new_name.to_string();
        self
    }

    pub fn persist(self) -> (String, Vec<u8>) {
        // A persisted file must not be deleted, so `drop` must not run. `ManuallyDrop` runs
        // neither `TempFile::drop` nor the destructors of the fields, so every field, the log
        // included, has to be moved out of it.
        let file = ManuallyDrop::new(self);
        // SAFETY: each field is read exactly once and `file` is never used again, so each value
        // ends up with exactly one owner: nothing is dropped twice, and nothing leaks.
        let (name, data, log) =
            unsafe { (ptr::read(&file.name), ptr::read(&file.data), ptr::read(&file.log)) };
        // This file's handle to the log is released here.
        drop(log);
        (name, data)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let line = format!("delete {} ({} bytes)", self.name, self.data.len());
        self.log.borrow_mut().push(line);
    }
}
