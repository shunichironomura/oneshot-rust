use std::cell::RefCell;
use std::rc::Rc;

pub struct Ledger {
    log: Rc<RefCell<Vec<String>>>,
}

pub struct Account {
    name: String,
    balance: u32,
    log: Rc<RefCell<Vec<String>>>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger {
            log: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn open(&self, name: &str) -> Account {
        Account {
            name: name.to_string(),
            balance: 0,
            log: Rc::clone(&self.log),
        }
    }

    pub fn log(&self) -> Vec<String> {
        self.log.borrow().clone()
    }

    pub fn open_accounts(&self) -> usize {
        // Every live account holds one more handle to the shared log.
        Rc::strong_count(&self.log) - 1
    }
}

impl Account {
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
        self.log.borrow_mut().push(format!("{} +{}", self.name, amount));
    }

    pub fn withdraw(&mut self, amount: u32) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        self.log.borrow_mut().push(format!("{} -{}", self.name, amount));
        true
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }
}
