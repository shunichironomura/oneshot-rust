use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub type Log = Rc<RefCell<Vec<String>>>;

pub struct TempFile {
    name: String,
    data: Vec<u8>,
    log: Log,
    /// Set by `persist`, so that `drop` does not delete the file.
    persisted: bool,
}

impl TempFile {
    pub fn create(name: &str, log: &Log) -> TempFile {
        log.borrow_mut().push(format!("create {name}"));
        TempFile { name: name.to_string(), data: Vec::new(), log: Rc::clone(log), persisted: false }
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

    pub fn persist(mut self) -> (String, Vec<u8>) {
        // `self` is still dropped at the end of this method, which also releases `log`.
        // `drop` only has to know that there is nothing to delete.
        self.persisted = true;
        (mem::take(&mut self.name), mem::take(&mut self.data))
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if !self.persisted {
            let line = format!("delete {} ({} bytes)", self.name, self.data.len());
            self.log.borrow_mut().push(line);
        }
    }
}
