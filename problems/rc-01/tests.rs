use crate::judge::{case, check, check_eq};
use crate::user::{Account, Ledger};

fn strs(xs: &[&str]) -> Vec<String> {
    xs.iter().map(|s| s.to_string()).collect()
}

fn open_pair() -> (Ledger, Account, Account) {
    let ledger = Ledger::new();
    let a = ledger.open("ann");
    let b = ledger.open("bob");
    (ledger, a, b)
}

pub fn run() {
    case("sample-1", || {
        let ledger = Ledger::new();
        let mut ann = ledger.open("ann");
        let mut bob = ledger.open("bob");
        ann.deposit(50);
        bob.deposit(10);
        check(ann.withdraw(20), "withdraw 20 from 50 should succeed")?;
        check(!bob.withdraw(30), "withdraw 30 from 10 should fail")?;
        check_eq((ann.balance(), bob.balance()), (30, 10))?;
        check_eq(ledger.log(), strs(&["ann +50", "bob +10", "ann -20"]))
    });
    case("sample-2", || {
        let ledger = Ledger::new();
        let _ann = ledger.open("ann");
        {
            let _bob = ledger.open("bob");
            check_eq(ledger.open_accounts(), 2)?;
        }
        check_eq(ledger.open_accounts(), 1)
    });
    case("hidden-1", || {
        let ledger = Ledger::new();
        check_eq(ledger.log(), Vec::<String>::new())?;
        check_eq(ledger.open_accounts(), 0)
    });
    case("hidden-2", || {
        // Accounts can be returned together with the ledger they came from.
        let (ledger, mut a, mut b) = open_pair();
        a.deposit(5);
        b.deposit(7);
        check(b.withdraw(7), "withdrawing the whole balance should succeed")?;
        check_eq(b.balance(), 0)?;
        check_eq(ledger.log(), strs(&["ann +5", "bob +7", "bob -7"]))
    });
    case("hidden-3", || {
        // An account keeps working after the ledger is gone, and the log is not reset.
        let ledger = Ledger::new();
        let mut a = ledger.open("ann");
        a.deposit(1);
        let snapshot = ledger.log();
        drop(ledger);
        a.deposit(2);
        check_eq(a.balance(), 3)?;
        check_eq(snapshot, strs(&["ann +1"]))
    });
    case("hidden-4", || {
        // Two accounts with the same name are still separate accounts.
        let ledger = Ledger::new();
        let mut x = ledger.open("sam");
        let mut y = ledger.open("sam");
        x.deposit(4);
        check(!y.withdraw(1), "the second account is empty")?;
        check_eq((x.balance(), y.balance()), (4, 0))?;
        let accounts: Vec<Account> = (0..3).map(|_| ledger.open("z")).collect();
        check_eq(ledger.open_accounts(), 5)?;
        drop(accounts);
        drop(x);
        check_eq(ledger.open_accounts(), 1)?;
        check_eq(ledger.log(), strs(&["sam +4"]))
    });
    case("hidden-5", || {
        // Separate ledgers do not share anything.
        let l1 = Ledger::new();
        let l2 = Ledger::new();
        let mut a = l1.open("a");
        let _b = l2.open("b");
        let _c = l2.open("c");
        a.deposit(9);
        check_eq((l1.open_accounts(), l2.open_accounts()), (1, 2))?;
        check_eq((l1.log(), l2.log()), (strs(&["a +9"]), Vec::new()))
    });
}
