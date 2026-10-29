use crate::judge::{case, check_eq};
use crate::user::{Bank, TransferError};

pub fn run() {
    case("example-1", || {
        let mut bank = Bank::new(&[("a", 10), ("b", 0)]);
        check_eq(bank.transfer("a", "b", 7), Ok(()))?;
        check_eq((bank.balance("a"), bank.balance("b")), (Some(3), Some(7)))?;
        check_eq(bank.transfer("a", "a", 1), Err(TransferError::SameAccount))
    });
    case("case-1", || {
        // SameAccount is checked first, even for an unknown name or an amount of 0.
        let mut bank = Bank::new(&[("a", 10), ("b", 5)]);
        check_eq(bank.transfer("zz", "zz", 1), Err(TransferError::SameAccount))?;
        check_eq(bank.transfer("b", "b", 0), Err(TransferError::SameAccount))?;
        check_eq(bank.transfer("a", "a", 100), Err(TransferError::SameAccount))?;
        check_eq((bank.balance("a"), bank.balance("b")), (Some(10), Some(5)))?;
        check_eq(bank.balance("zz"), None)
    });
    case("case-2", || {
        // An unknown `to` must not take money out of `from`.
        let mut bank = Bank::new(&[("alice", 50), ("bob", 20)]);
        check_eq(bank.transfer("alice", "carol", 30), Err(TransferError::UnknownAccount))?;
        check_eq(bank.transfer("carol", "bob", 0), Err(TransferError::UnknownAccount))?;
        check_eq(bank.transfer("alice", "dave", 0), Err(TransferError::UnknownAccount))?;
        check_eq(
            (bank.balance("alice"), bank.balance("bob"), bank.balance("carol")),
            (Some(50), Some(20), None),
        )
    });
    case("case-3", || {
        // UnknownAccount wins over InsufficientFunds.
        let mut bank = Bank::new(&[("a", 3), ("b", 0)]);
        check_eq(bank.transfer("a", "x", 5), Err(TransferError::UnknownAccount))?;
        check_eq(bank.transfer("x", "b", 5), Err(TransferError::UnknownAccount))?;
        check_eq(bank.transfer("b", "x", 1), Err(TransferError::UnknownAccount))?;
        check_eq((bank.balance("a"), bank.balance("b")), (Some(3), Some(0)))
    });
    case("case-4", || {
        // A failed transfer changes nothing; the exact balance and an amount of 0 are fine.
        let mut bank = Bank::new(&[("a", 5), ("b", 1), ("c", 0)]);
        check_eq(bank.transfer("a", "b", 6), Err(TransferError::InsufficientFunds))?;
        check_eq((bank.balance("a"), bank.balance("b")), (Some(5), Some(1)))?;
        check_eq(bank.transfer("c", "a", 0), Ok(()))?;
        check_eq(bank.transfer("a", "b", 5), Ok(()))?;
        check_eq(bank.transfer("b", "c", 4), Ok(()))?;
        check_eq(bank.transfer("a", "c", 1), Err(TransferError::InsufficientFunds))?;
        check_eq(
            (bank.balance("a"), bank.balance("b"), bank.balance("c")),
            (Some(0), Some(2), Some(4)),
        )
    });
}
