複数の口座が 1 つの台帳に記録します。`Ledger` と `Account` を実装してください。

```rust
pub struct Ledger { /* 自由に定義 */ }
pub struct Account { /* 自由に定義 */ }

impl Ledger {
    pub fn new() -> Self;
    pub fn open(&self, name: &str) -> Account;
    pub fn log(&self) -> Vec<String>;
    pub fn open_accounts(&self) -> usize;
}

impl Account {
    pub fn deposit(&mut self, amount: u32);
    pub fn withdraw(&mut self, amount: u32) -> bool;
    pub fn balance(&self) -> u32;
}
```

- `open` は残高 0 の口座を新しく作ります。名前が同じでも、口座ごとに残高は別です。
- `deposit` は `amount` を加え、台帳のログに `"<name> +<amount>"` を記録します。
- `withdraw` は `amount` を引いて `"<name> -<amount>"` を記録し、`true` を返します。
  残高が `amount` より少なければ何も変えず、何も記録せずに `false` を返します。
- `log` は、この台帳の口座が記録したものを古い順にすべて返します。
- `open_accounts` は、この台帳から開いた口座のうち、まだ存在している（drop されていない）ものの数を返します。
- 口座はどこへムーブしてもよく、台帳より長く生き残ることもあります。

### 例

```rust
let ledger = Ledger::new();
let mut ann = ledger.open("ann");
let mut bob = ledger.open("bob");
ann.deposit(50);
bob.deposit(10);
ann.withdraw(20); // true
bob.withdraw(30); // false
// ledger.log() == ["ann +50", "bob +10", "ann -20"]
// ledger.open_accounts() == 2
```
