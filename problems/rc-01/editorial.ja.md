`Account` は `&Ledger` を持てません。口座は台帳から離れた場所へムーブされたり、台帳より長く生き残ったりしますし、
`open(&self, ...) -> Account` というシグネチャには両者を結びつけるライフタイムもありません。
そこで、台帳と口座が `Rc` でログを**共同所有**します。`Rc::clone` は同じ `Vec` を指すハンドルを 1 つ増やすだけです。

`Rc` からは共有参照しか得られませんが、`deposit` はログに書き込む必要があります。`RefCell` を使うと借用チェックが実行時に移り、
共有ハンドルからでも `borrow_mut()` で可変借用を得られます（ほかの借用が残っていればパニックします）。
`self.log.borrow_mut().push(...)` のように、借用はなるべく短くします。

`Rc::strong_count` は、まだ残っているハンドルの数を返します。台帳が 1 つ、生きている口座がそれぞれ 1 つ持っているので、
`open_accounts` はこの数から 1 を引いた値です。口座を drop するとハンドルも drop されるので、数は自動で減ります。

### 別解

口座の数を自分で数え、`Drop` の実装で減らす方法です。共有する状態を 1 つの構造体にまとめると、`RefCell` も 1 つで済みます。

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Shared {
    log: Vec<String>,
    open: usize,
}

pub struct Ledger {
    shared: Rc<RefCell<Shared>>,
}

pub struct Account {
    name: String,
    balance: u32,
    shared: Rc<RefCell<Shared>>,
}

impl Ledger {
    pub fn new() -> Self {
        Ledger { shared: Rc::default() }
    }

    pub fn open(&self, name: &str) -> Account {
        self.shared.borrow_mut().open += 1;
        Account { name: name.to_string(), balance: 0, shared: Rc::clone(&self.shared) }
    }

    pub fn log(&self) -> Vec<String> {
        self.shared.borrow().log.clone()
    }

    pub fn open_accounts(&self) -> usize {
        self.shared.borrow().open
    }
}

impl Account {
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
        self.shared.borrow_mut().log.push(format!("{} +{}", self.name, amount));
    }

    pub fn withdraw(&mut self, amount: u32) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        self.shared.borrow_mut().log.push(format!("{} -{}", self.name, amount));
        true
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }
}

impl Drop for Account {
    fn drop(&mut self) {
        self.shared.borrow_mut().open -= 1;
    }
}
```
