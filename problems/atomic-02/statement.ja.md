次のスピンロックはコンパイルが通り、手元で実行すると「それっぽく」動きます。
しかし**未定義動作**（UB）を含んでおり、Miri で実行すると UB として検出されます。

`SpinLock<T>` は `T` 型の値を守り、一度に 1 つのスレッドだけが使えるようにするロックです。
`std::sync::Mutex` と違って、待っているスレッドを眠らせることはなく、ロックが空くまで
試し続けます（スピンします）。実装はすべてエディタに入っています。

```rust
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self;
    pub fn lock(&self) -> SpinGuard<'_, T>;
    pub fn try_lock(&self) -> Option<SpinGuard<'_, T>>;
    pub fn into_inner(self) -> T;
}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

// SpinGuard<'_, T> は Deref<Target = T>、DerefMut、Drop を実装しています。
```

- ガードを通して値にアクセスできます。`*guard` で読み、`*guard = ...` や
  `guard.push_str(...)` のようなメソッド呼び出しで変更します。複数のスレッドが同時に `lock` や
  `try_lock` を呼んでも、1 つのロックについて生きているガードは常に高々 1 つです。
- ガードを通した変更は、どのスレッドが持つかにかかわらず、同じロックのそれ以降のすべての
  ガードから見えます。
- `lock` はロックが空くまで待ってからロックを取り、ガードを返します。試行と試行の間には
  `std::hint::spin_loop()` を呼びます。
- `try_lock` は待ちません。呼んだ時点でロックが空いていれば、ロックを取って `Some(guard)` を
  返します。`None` を返すのは、そのロックのガードが（呼び出したスレッドを含め、どこかの
  スレッドで）生きている間だけです。
- ガードを破棄すると、ロックが空きます。
- `new` は `const fn` なので、`SpinLock` を `static` に置けます。
- `into_inner` はロックを消費して、中の値を返します。

UB を取り除いて、説明どおりに動くようにしてください。`SpinLock` と `SpinGuard` のフィールドは
変えず、`std::sync` のロック（`Mutex` など）は使わないでください（`UnsafeCell` の中の値に
触るには、引き続き `unsafe` が必要です）。

### 例

```rust
let counter = SpinLock::new(0);
thread::scope(|s| {
    for _ in 0..3 {
        s.spawn(|| {
            for _ in 0..5 {
                *counter.lock() += 1;
            }
        });
    }
});
assert_eq!(counter.into_inner(), 15);

let lock = SpinLock::new(String::from("spin"));
let mut guard = lock.lock();
guard.push_str("lock");
assert!(lock.try_lock().is_none()); // ガードはまだ生きている
drop(guard);
assert_eq!(lock.try_lock().map(|g| g.len()), Some(8));
```
