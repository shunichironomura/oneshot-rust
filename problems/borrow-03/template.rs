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
        if from == to {
            return Err(TransferError::SameAccount);
        }
        let src = self.accounts.get_mut(from).ok_or(TransferError::UnknownAccount)?;
        let dst = self.accounts.get_mut(to).ok_or(TransferError::UnknownAccount)?;
        if *src < amount {
            return Err(TransferError::InsufficientFunds);
        }
        *src -= amount;
        *dst += amount;
        Ok(())
    }
}

pub fn merge_accounts(balances: &mut [u64], from: usize, to: usize) {
    let src = &mut balances[from];
    let dst = &mut balances[to];
    *dst += *src;
    *src = 0;
}
