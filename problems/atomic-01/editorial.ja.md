`static mut` は、どのスレッドからも同期なしで読み書きできるグローバル変数です。
エディション 2024 では、これへの参照を作ること（`&NEXT`、`&mut NEXT`、借用するメソッド呼び出し）は
拒否されますが、テンプレートのように値として読んだり代入したりするコードは、今でもコンパイルが通ります。
2 つのスレッドが同時に `let id = NEXT; NEXT = id + 1;` を実行すると、同じメモリへの書き込みと
読み込みの順序が何によっても決まりません。これが**データ競合**で、Rust では UB です。
実際には 2 つのスレッドが同じ値を読んで同じ ID を払い出すことがありますが、そうならなくても
プログラムは UB であり、2 つのスレッドが同期なしに `NEXT` を使った時点で Miri が報告します。

修正に `unsafe` は要りません。`AtomicU64` は共有参照越しに変更できるので、普通の `static` に置けます
（`AtomicU64::new` は `const fn` です）。`fetch_add` は値に足し込み、**足す前の**値を返します。
これは分割できない 1 つの read-modify-write 操作で、読み込みと書き込みの間にほかのスレッドが
割り込めないので、呼び出しごとに必ず違う値が返ります。`reserve(n)` も同じように `fetch_add(n)` を使い、
`start..start + n` のブロック全体を一度に確保します。

`Ordering::Relaxed` で十分です。1 つのアトミック変数への変更はすべて 1 つの全順序（変更順序）に並び、
各 `fetch_add` はその順序で直前に書かれた値を読むので、それだけで結果は重複しません。
また、ある操作が同じスレッドの以前の操作より前に並ぶことはないので、各スレッドが受け取る値は増加します。
より強い順序付けが必要になるのは、カウンタを使ってほかのデータもスレッド間で受け渡すときだけです。

詰まりやすい点：

- `let start = NEXT.load(..); NEXT.store(start + n, ..);` は、どちらの操作もアトミックなので
  データ競合ではありませんが、2 つの操作の間にほかのスレッドが割り込めるので、2 つのスレッドが
  重なった範囲を受け取ります。`reserve` の中で `next_id` を `n` 回呼ぶのもだめです。
  途中でほかのスレッドが ID を受け取ると、その ID が返す範囲の中に入ってしまいます。
- `static mut NEXT: AtomicU64` にして `unsafe { NEXT.fetch_add(..) }` と書くと、
  エディション 2024 ではコンパイルが通りません。メソッド呼び出しが `static mut` への共有参照を作るからです。
- `fetch_add` が返すのは**足す前の**値です。足した後の値だと思って使うと、以前に払い出した ID と
  重なる範囲を返してしまいます。

### 別解

`Mutex` を使っても、read-modify-write を分割できない操作にできます。`Mutex::new` も `const fn` なので、
`static` に置けます。ここでは `next_id` を `reserve` を使って書いています。
ロックはアトミック操作よりコストがかかりますが、1 つのアトミック変数に収まらない状態にも使えます。

```rust
use std::ops::Range;
use std::sync::Mutex;

static NEXT: Mutex<u64> = Mutex::new(0);

pub fn next_id() -> u64 {
    reserve(1).start
}

pub fn reserve(n: u64) -> Range<u64> {
    let mut next = NEXT.lock().unwrap();
    let start = *next;
    *next += n;
    start..start + n
}
```

`try_update`（Rust 1.95 で安定化。以前の名前 `fetch_update` は非推奨）は、間にほかのスレッドが値を
変えていなければ成功する compare-and-swap のループで、クロージャを再試行します。新しい値を検査したいとき、
たとえばオーバーフローを拒否したいときに便利です。

```rust
pub fn reserve(n: u64) -> Range<u64> {
    let start = NEXT
        .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| next.checked_add(n))
        .expect("ran out of ids");
    start..start + n
}
```
