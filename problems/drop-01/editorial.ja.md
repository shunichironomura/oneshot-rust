ローカル変数は宣言と**逆の**順に破棄されますが、`Vec` の要素は**先頭から**順に破棄されます
（構造体のフィールドや、配列・タプルの要素も同じです）。そのため `drop(locks)` と書いても、
単に `locks` をスコープの外に出しても、`a`、`b`、`c` の順に解放されてしまい、`release_all` としては
誤りです。代わりに末尾から `pop` していきます。

もう 1 つの落とし穴は `with_lock` にあります。`let _ = Lock::acquire(name, log);` は関数の終わりまで
ロックを持ち続けるように見えますが、`_` は変数ではありません。このパターンは値に一致するだけで
束縛はしないので、新しいロックを所有するものがなく、ロックは文の終わりで、つまり `f` を呼ぶ前に
破棄されてしまいます。式文 `Lock::acquire(name, log);` も同じです。名前の付いた変数なら、スコープの
終わりまでロックを所有します。先頭にアンダースコアを付けてもこれは変わりません。`_lock` は本物の変数で、
アンダースコアは「使われていない変数」の警告を消すだけです。末尾の式 `f()` はローカル変数が破棄される
前に評価されるので、その結果をそのまま返せます。ロックは `f` から戻った直後に解放されます。

`swap_in` には特別な工夫は要りません。`*slot = Lock::acquire(name, &slot.log)` では右辺が先に
評価され、その後で代入が `*slot` の古い値を破棄します。戻り値の型 `Lock` は何も借用していない
（新しいロックは `Rc` の複製を自分で持つ）ので、`slot.log` の共有借用は `acquire` から戻った
時点で終わり、`*slot` に代入できます。

`keep_first` も `release_all` と同じ「先頭から」の規則を使いますが、今度はこれが求める順番です。
`into_iter()` でロックをイテレータに移し、`next()` で最初のロックを取り出します。残りのロックは
イテレータが持ったままで、関数の終わりにイテレータが破棄されるときに、先頭から順に破棄されます。
`swap_remove(0)` は最後のロックを先頭に移すので注意してください。

### 別解

`with_lock` では、ロックを明示的に解放して順番を見えるようにすることもできます。

```rust
pub fn with_lock<R>(name: &str, log: &Log, f: impl FnOnce() -> R) -> R {
    let lock = Lock::acquire(name, log);
    let result = f();
    drop(lock);
    result
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
