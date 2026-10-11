use std::cell::RefCell;
use std::mem;
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

    pub fn rename(self, new_name: &str) -> TempFile {
        self.log.borrow_mut().push(format!("rename {} -> {new_name}", self.name));
        TempFile { name: new_name.to_string(), ..self }
    }

    pub fn persist(self) -> (String, Vec<u8>) {
        let name = self.name;
        let data = self.data;
        // A persisted file must not be deleted, so `drop` must not run.
        mem::forget(self);
        (name, data)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let line = format!("delete {} ({} bytes)", self.name, self.data.len());
        self.log.borrow_mut().push(line);
    }
}
