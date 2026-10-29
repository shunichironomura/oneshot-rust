use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TransferError {
    UnknownAccount,
    SameAccount,
    InsufficientFunds,
}

pub struct Bank {
    accounts: HashMap<String, u64>,
}

impl Bank {
    pub fn new(accounts: &[(&str, u64)]) -> Self {
        Bank {
            accounts: accounts
                .iter()
                .map(|&(name, balance)| (name.to_string(), balance))
                .collect(),
        }
    }

    pub fn balance(&self, name: &str) -> Option<u64> {
        self.accounts.get(name).copied()
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
        // Must come first: `get_disjoint_mut` panics when both keys are the same account.
        if from == to {
            return Err(TransferError::SameAccount);
        }
        let [Some(src), Some(dst)] = self.accounts.get_disjoint_mut([from, to]) else {
            return Err(TransferError::UnknownAccount);
        };
        if *src < amount {
            return Err(TransferError::InsufficientFunds);
        }
        *src -= amount;
        *dst += amount;
        Ok(())
    }
}
