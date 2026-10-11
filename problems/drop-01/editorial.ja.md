ローカル変数は宣言と**逆の**順に破棄されますが、`Vec` の要素は**先頭から**順に破棄されます
（構造体のフィールドや、配列・タプルの要素も同じです）。そのため `drop(locks)` と書いても、
単に `locks` をスコープの外に出しても、`a`、`b`、`c` の順に解放されてしまい、`release_all` としては
誤りです。代わりに末尾から `pop` していきます。

`swap_in` には特別な工夫は要りません。`*slot = Lock::acquire(name, &slot.log)` では右辺が先に
評価され、その後で代入が `*slot` の古い値を破棄します。新しいロックは `Rc` の複製を自分で持つので、
`slot.log` の借用は `acquire` から戻った時点で終わります。

`keep_first` も同じ「先頭から」の規則を使いますが、今度はこれが求める順番です。`into_iter()` で
ロックをイテレータに移し、`next()` で最初のロックを取り出します。残りのロックはイテレータが
持ったままで、関数の終わりにイテレータが破棄されるときに、先頭から順に破棄されます。
`swap_remove(0)` は最後のロックを先頭に移すので注意してください。

落とし穴は `into_name` です。`self.name` を返すコードはコンパイルが通りません。`Drop` を実装した
型からは、フィールドをムーブで取り出せないからです（E0509）。値には後で `drop` が実行され、
そのときすべてのフィールドがそろっている必要があります。コンパイラは `self.name.clone()` を
提案しますが、これだとコンパイルは通るものの、`self` が破棄されるときにロックが解放されて
しまいます。名前を取り出し、`drop` が見分けられるものを代わりに残す必要があります。模範解答では
フィールドを `Option<String>` にしています。`take()` は `None` を残し、`drop` は `None` なら
何も記録しません。

`std::mem::forget(self)` もコンパイルは通り、`drop` を飛ばせますが、フィールドの破棄も飛ばして
しまいます。`log` の `Rc` が解放されないまま残るので、`Rc::strong_count(&log)` は 1 に戻らず、
Miri はメモリリークを報告します。`std::mem::take` で名前を取り出し、`drop` で空の名前を
読み飛ばす方法も不十分です。空文字列も正しい名前だからです。

### 別解

`name: String` はそのままにして、フラグを足す方法です。`std::mem::take` は空の `String` を残し、
ロックが消費されたことはフラグで `drop` に伝えます。

```rust
pub struct Lock {
    name: String,
    log: Log,
    armed: bool,
}

impl Lock {
    pub fn acquire(name: &str, log: &Log) -> Lock {
        log.borrow_mut().push(format!("acquire {name}"));
        Lock { name: name.to_string(), log: Rc::clone(log), armed: true }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn into_name(mut self) -> String {
        self.armed = false;
        std::mem::take(&mut self.name)
    }
}

impl Drop for Lock {
    fn drop(&mut self) {
        if self.armed {
            self.log.borrow_mut().push(format!("release {}", self.name));
        }
    }
}
```

`release_all` は、逆順のイテレータを使って破棄することもできます。

```rust
pub fn release_all(locks: Vec<Lock>) {
    locks.into_iter().rev().for_each(drop);
}
```

`std::mem::replace` を使うと、`swap_in` の代入がしていることを明示的に書けます。

```rust
pub fn swap_in(slot: &mut Lock, name: &str) {
    let new = Lock::acquire(name, &slot.log);
    let old = std::mem::replace(slot, new);
    drop(old);
}
```
