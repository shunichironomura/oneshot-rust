pub struct Ledger {
    // Add the fields you need.
}

pub struct Account {
    // Add the fields you need.
}

impl Ledger {
    pub fn new() -> Self {
        todo!()
    }

    pub fn open(&self, name: &str) -> Account {
        todo!()
    }

    pub fn log(&self) -> Vec<String> {
        todo!()
    }

    pub fn open_accounts(&self) -> usize {
        todo!()
    }
}

impl Account {
    pub fn deposit(&mut self, amount: u32) {
        todo!()
    }

    pub fn withdraw(&mut self, amount: u32) -> bool {
        todo!()
    }

    pub fn balance(&self) -> u32 {
        todo!()
    }
}
