`std::sync::MutexGuard` は `Send` ではありません。ミューテックスのロックを、ロックしたスレッド自身が
解除しなければならないプラットフォームがあるからです。`async fn` の Future は `.await` の時点で
生きている値をすべて保持するので、そこでガードが生きていると Future 全体が `Send` でなくなり、
ハーネスはそれを別のスレッドに移せません（「future cannot be sent between threads safely」）。
これは実際のバグでもあります。Future が待っている間、ロックを取ろうとするほかのタスクはスレッドごと
止まってしまい、同じスレッドのタスクならデッドロックします。2 つの関数は、それぞれ別の形で
`.await` をまたいでガードを持っています。

**`record_all`**：`guard` はループ本体の終わり、つまり `yield_now().await` の後まで生きています。
すぐに思いつく `.await` の直前で `drop(guard);` する修正は、ロックは間に合うように解除しますが、
それでもコンパイルが通りません。Future に何を保持するかを決めるとき、コンパイラは `.await` の後でも
必要になりうる変数を調べますが、借用された変数については保守的です。一度借用された変数
（`guard.push(item)` や `guard.len()` は、`DerefMut` や `Deref` を通して `guard` を借用します）は、
参照がまだ残っているかもしれないので、スコープの終わりまで必要とみなされます。`drop(guard)` は
ガードをムーブして手放しますが、変数は `.await` の時点でまだスコープ内にあるので、
`MutexGuard` は依然として Future の一部とみなされます。`.await` の前に終わるブロックの中に
ガードを入れてください。ブロックの値が新しい長さになり、ガードのスコープはそこで終わります。

**`forward_all`**：ここにはガードの変数はありませんが、`inbox.lock().unwrap()` が一時的なガードを
作ります。一時的な値は、それを含む文の終わりまで生きます。本体は `.await` で終わる 1 つの式なので、
`record_all` を実行している間ずっと `inbox` はロックされたままです。メッセージの取り出しを独立した
`let` 文に分ければ、一時的な値はその `;` で破棄されます。`let len = record_all(log, take(...)).await;`
と書くだけでは足りません。`.await` が同じ文の中にあるからです。`.await` を含まない文の中の
一時的なガード（`let mut len = log.lock().unwrap().len();` など）は問題ありません。

`log.lock().unwrap().push(item); len = log.lock().unwrap().len();` のように 2 回ロックする書き方も
コンパイルは通りますが、2 回のロックの間にほかのスレッドが要素を追加するかもしれないので、`len` が
この追加の直後の長さになるとは限りません。Tokio などの非同期ランタイムには、ガードを持ったまま
`.await` してよい独自のミューテックスがありますが、`std` のガードを持ったまま `.await` しては
いけません。

### 別解

ロックを、普通の（async でない）関数の中で取る方法です。その関数をどう書いても、ガードが `.await` の
時点で生きていることはありません。

```rust
fn push_and_len(log: &Mutex<Vec<String>>, item: String) -> usize {
    let mut guard = log.lock().unwrap();
    guard.push(item);
    guard.len()
}

pub async fn record_all(log: &Mutex<Vec<String>>, items: Vec<String>) -> usize {
    let mut len = log.lock().unwrap().len();
    for item in items {
        len = push_and_len(log, item);
        yield_now().await;
    }
    len
}
```

ブロック式ではなく、ブロック文でもかまいません。

```rust
for item in items {
    {
        let mut guard = log.lock().unwrap();
        guard.push(item);
        len = guard.len();
    }
    yield_now().await;
}
```

エディション 2024 からは、ブロックの最後の式の一時的な値はブロックの終わりで破棄されるので、
`forward_all` の引数をブロックで囲むだけでも十分です（エディション 2021 では外側の文の終わりまで
生きていたので、これではコンパイルが通りませんでした）。

```rust
pub async fn forward_all(inbox: &Mutex<Vec<String>>, log: &Mutex<Vec<String>>) -> usize {
    record_all(log, { std::mem::take(&mut *inbox.lock().unwrap()) }).await
}
```
