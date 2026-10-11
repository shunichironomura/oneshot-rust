共有参照 `&T` は、それが使われている間は指す先のメモリが変わらないことを約束します。
唯一の例外は `UnsafeCell` の中にあるメモリです。これは `&` そのものに課される規則で、どこかに `&mut` が
あるかどうかとは関係ありません。そのため、テンプレートの `SAFETY` コメントは的外れです。
`&raw const self.hits as *mut u32` を通して書き込むのは、そのフィールドを指す参照がほかにまったくなくても UB です。
コンパイラはこの規則を前提にしてよく、たとえば関数呼び出しをまたいで `self.hits` をレジスタに置いたままにできます。
Miri のエラーも同じことを指摘しています。`&raw const self.hits` は `&self` からポインタを作るので、そのタグは
「SharedReadOnly（共有の読み取り専用）」の権限しか与えません。`*mut` にキャストしてもそれは変わりません。

解決策は**内部可変性**です。`UnsafeCell` を包み、`&self` 越しに中身を変える安全な API を提供する型を使います。

- 回数には `Cell<u32>` を使います。`get` は値をコピーして取り出し、`set` は値を置き換えます。
  `Cell` は `&self` 越しに中身への参照を渡すことがないので、`&self` 越しに中身を変更しても安全です。
- 合計のキャッシュには `OnceCell<u32>` を使います。`get_or_init` は、セルが空のときだけクロージャを実行し、
  格納した値への `&u32` を返します（そのため `*` が必要です）。クロージャが実行されるのはちょうど合計を計算する
  ときなので、`computations` はそこで数えます。
- `insert` は `&mut self` を受け取るので、特別なことは要りません。セルを空の `OnceCell::new()` に置き換えます
  （`self.total.take()` でも構いません）。

`get` と `total` を `&mut self` に変えれば素直に `self.hits += 1` と書けますが、呼び出し側が動かなくなります。
同時に存在する 2 つの `&Lookup` や `Rc<Lookup>` からは、共有アクセスしかできないからです。
また、`0` を「まだ計算していない」の意味に使わないよう注意してください。表の合計は `0` になることがあり、
その場合も計算は 1 回だけでなければなりません。

`Cell` を持つ `Lookup` は `Sync` ではなくなるので、スレッド間では共有できません。この問題ではそれで構いません。

### 別解

`Option<u32>` は `Copy` なので、キャッシュを `Cell<Option<u32>>` で持つこともできます。
1 を足すには `Cell::update` を使うと短く書けます。

```rust
use std::cell::Cell;
use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: Cell<u32>,
    misses: Cell<u32>,
    total: Cell<Option<u32>>,
    computations: Cell<u32>,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: Cell::new(0),
            misses: Cell::new(0),
            total: Cell::new(None),
            computations: Cell::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.total.set(None);
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() { &self.hits } else { &self.misses };
        counter.update(|n| n + 1);
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits.get()
    }

    pub fn misses(&self) -> u32 {
        self.misses.get()
    }

    pub fn total(&self) -> u32 {
        if let Some(total) = self.total.get() {
            return total;
        }
        let total = self.table.values().sum();
        self.total.set(Some(total));
        self.computations.update(|n| n + 1);
        total
    }

    pub fn computations(&self) -> u32 {
        self.computations.get()
    }
}
```

`Lookup` をスレッド間で共有したいなら、スレッドセーフな型を使います。回数には `AtomicU32`、キャッシュには
`OnceLock<u32>` です。これらも `UnsafeCell` の上に作られています。

```rust
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: AtomicU32,
    misses: AtomicU32,
    total: OnceLock<u32>,
    computations: AtomicU32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: AtomicU32::new(0),
            misses: AtomicU32::new(0),
            total: OnceLock::new(),
            computations: AtomicU32::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.total.take();
    }

    pub fn get(&self, key: &str) -> Option<u32> {
        let value = self.table.get(key).copied();
        let counter = if value.is_some() { &self.hits } else { &self.misses };
        counter.fetch_add(1, Ordering::Relaxed);
        value
    }

    pub fn hits(&self) -> u32 {
        self.hits.load(Ordering::Relaxed)
    }

    pub fn misses(&self) -> u32 {
        self.misses.load(Ordering::Relaxed)
    }

    pub fn total(&self) -> u32 {
        *self.total.get_or_init(|| {
            self.computations.fetch_add(1, Ordering::Relaxed);
            self.table.values().sum()
        })
    }

    pub fn computations(&self) -> u32 {
        self.computations.load(Ordering::Relaxed)
    }
}
```
