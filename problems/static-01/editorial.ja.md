3 つの static のうち 2 つが、それぞれ別の理由でエラーになります。`NAMES` はこのままで問題ありません。

- **static の初期化式はコンパイル時に評価される**ので、呼べるのは `const fn` だけです。
  `Mutex::new`、`RwLock::new`、`Vec::new` は `const fn` ですが、`HashMap::new` は違います。
  `HashMap::new` は `RandomState` を作り、`RandomState` はプログラムの実行時にランダムなハッシュの鍵を
  選ぶからです（E0015）。`LazyLock` を使うと、初期化を実行時に回せます。クロージャは最初に使われたときに、
  最初にたどり着いたスレッドで一度だけ実行されます。static は中の値に参照外しされるので、
  `IDS.lock()` はそのまま使えます。
- **static は `Sync` でなければなりません**。どのスレッドからも同時に触れるからです。`Cell` は `Sync`
  ではありません（E0277）。2 つのスレッドが同時に `CALLS.set(CALLS.get() + 1)` を実行すると
  データ競合になります。`AtomicUsize::fetch_add` は、分割できない 1 回の操作で値を増やします。
  このカウンタはほかのデータを守っているわけではないので、`Ordering::Relaxed` で十分です。
  スレッドでの加算は、そのスレッドを `join` したスレッドから見えるようになります。

`static mut` や `thread_local!` では解決しません。`static mut` には `unsafe` が必要です（この問題では
禁止されていますし、Rust 2024 では `static mut` への参照を作ること自体が拒否されます）。`thread_local!` は
コンパイルが通りますが、スレッドごとに別々の表とカウンタができてしまいます。スレッドが違うと ID が
衝突し、`intern_calls` は今のスレッドでの呼び出ししか数えません。

`intern` のそれ以外の部分は最初から正しく、そのまま残すべきです。`IDS` は、検索してから新しい文字列が
両方の表に入るまでロックされたままなので、2 つのスレッドが 1 つの文字列に別々の ID を付けることは
ありませんし、次の ID は常に `names.len()` です。検索と挿入で `IDS` を別々にロックすると、
2 つのスレッドがどちらも検索で見つからずに進んでしまうことがあります。ロックは常に `IDS`、`NAMES` の
順に取るので、デッドロックもしません。

### 別解

`BTreeMap::new` は `const fn` なので、（`std::collections` の）`BTreeMap` なら `LazyLock` は要りません。
`HashMap::with_hasher` も `const fn` です。`BuildHasherDefault::<DefaultHasher>::new()` のように、
ランダムな鍵を選ばないハッシャーと組み合わせれば使えます。

```rust
static IDS: Mutex<BTreeMap<String, u32>> = Mutex::new(BTreeMap::new());
```

`OnceLock` は `LazyLock` を手動にしたものです。初期化の方法は、値を使う場所で渡します。

```rust
static IDS: OnceLock<Mutex<HashMap<String, u32>>> = OnceLock::new();

pub fn intern(s: &str) -> u32 {
    CALLS.fetch_add(1, Ordering::Relaxed);
    let table = IDS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut ids = table.lock().unwrap();
    // ... 残りは同じ
}
```

すべての状態を 1 つの `Mutex` に入れれば、更新の途中の状態がほかのスレッドから見えることはなく、
ロックの順序を気にする必要もありません。その代わり、読み出し同士も待ち合うことになります
（`RwLock` ならそれを避けられます）。

```rust
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
struct Interner {
    ids: HashMap<String, u32>,
    names: Vec<String>,
    calls: usize,
}

static INTERNER: LazyLock<Mutex<Interner>> = LazyLock::new(|| Mutex::new(Interner::default()));

pub fn intern(s: &str) -> u32 {
    let mut table = INTERNER.lock().unwrap();
    table.calls += 1;
    if let Some(&id) = table.ids.get(s) {
        return id;
    }
    let id = table.names.len() as u32;
    table.names.push(s.to_string());
    table.ids.insert(s.to_string(), id);
    id
}

pub fn resolve(id: u32) -> Option<String> {
    INTERNER.lock().unwrap().names.get(id as usize).cloned()
}

pub fn interned_count() -> usize {
    INTERNER.lock().unwrap().names.len()
}

pub fn intern_calls() -> usize {
    INTERNER.lock().unwrap().calls
}
```
