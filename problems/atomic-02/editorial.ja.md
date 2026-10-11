アトミックなフラグ `locked` そのものが競合することはありませんが、`UnsafeCell` の中の値は普通の
メモリです。スレッド A の後にスレッド B がロックを取るとき、A が値に書き込んだことが B のアクセスより
**前に起こる（happens-before）**関係になければ、2 つのアクセスはデータ競合になり、UB です。
`Ordering::Relaxed` は `locked` への各操作をアトミックにするだけで、ほかのメモリアクセスの順序は
何も保証しません。B は、A がロックを外す前に書いた内容を見ないまま `locked == false` を見ることが
あります。Miri は happens-before 関係を追跡していて、`deref_mut` が `&mut T` を作るところで競合を
報告します（参照を作ることもアクセスとして扱われます）。

直し方は、定番のメモリ順序の組み合わせです。

- ロックを外すのは **Release** のストアです。クリティカルセクションでしたことすべてが、`false` と
  一緒に公開されます。
- ロックを取るのは、その `false` を読む **Acquire** の操作です。Release が公開した内容がすべて、
  新しくロックを持ったスレッドから見えるようになります。

もう 1 つのバグは `try_lock` にあります。`load` して `store` するのは、別々の 2 つの手順です
（「確かめてから立てる」）。2 つのスレッドがどちらも `true` をストアする前に `false` を読むと、
両方がガードを手に入れてしまいます。メモリ順序を強くしてもこの隙間は埋まらず、`Relaxed` を
すべて `SeqCst` にしても残ります。確かめることと立てることは、**1 つの**アトミックな
read-modify-write にする必要があります。`compare_exchange(false, true, Acquire, Relaxed)` は、
まさにその瞬間に値が `false` であるときだけ `true` に変え、変えたかどうかを返します。
大事なのは成功時の順序 `Acquire` です。失敗したときはロックを取れておらず、守られた値には
触らないので `Relaxed` で十分です。

詰まりやすい点です。

- Acquire は読む操作に、Release は書く操作に使います。`store(true, Acquire)` や
  `load(Release)` は実行時にパニックします（"there is no such thing as an acquire store"）。
  `compare_exchange` の失敗時の順序に `Release` や `AcqRel` を指定した場合も同じです。
- `compare_exchange_weak` は、値が `false` でも失敗することがあります（見せかけの失敗）。
  `lock` の再試行ループの中なら問題ありませんが、これで作った `try_lock` は空いているロックに
  対して `None` を返すことがあります。Miri はこの見せかけの失敗を頻繁に起こします。
- 確かめることと立てることを 1 つの操作にすれば `SeqCst` でも動きますが、ロックに必要なのは
  取るときの Acquire と外すときの Release だけです（名前の由来もここにあります）。

2 つの `unsafe impl Sync` はもともと正しく書けていました。`SpinLock<T>` は `&mut T` を一度に
1 つのスレッドにだけ渡すので、`Mutex<T>` と同じく、`T: Send` ならスレッド間で共有できます。
共有された `&SpinGuard` は、それを持つすべてのスレッドに `&T` を渡すので、ガードを共有できるのは
`T: Sync` のときだけです。この行がないと、ガードは `&SpinLock<T>` しか持っていないので、
`T: Send` でさえあれば `Sync` になってしまいます。

### 別解

`swap(true, Acquire)` も、確かめることと立てることを 1 回で行います。`true` を書き込んで、元の値を
返します。元の値がすでに `true` だったなら、何も変わっていません。

```rust
pub fn try_lock(&self) -> Option<SpinGuard<'_, T>> {
    if self.locked.swap(true, Ordering::Acquire) {
        None
    } else {
        Some(SpinGuard { lock: self })
    }
}
```

`lock` は、普通のロードで待ち、ロックが空いたように見えたときだけ read-modify-write を試す
こともできます（test and test-and-set）。実際のハードウェアでは、待っているスレッドどうしが
キャッシュラインを奪い合わずに済みます。ここでは `compare_exchange_weak` で問題ありません。
見せかけの失敗は、ループがもう 1 周するだけだからです。内側のループのロードは `Relaxed` で
構いません。大事な Acquire は、成功した `compare_exchange_weak` のものです。

```rust
pub fn lock(&self) -> SpinGuard<'_, T> {
    while self
        .locked
        .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        while self.locked.load(Ordering::Relaxed) {
            hint::spin_loop();
        }
    }
    SpinGuard { lock: self }
}
```
