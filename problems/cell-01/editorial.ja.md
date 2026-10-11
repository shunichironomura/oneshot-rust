共有参照 `&T` は、それが使われている間は指す先のメモリが変わらないことを約束します。
唯一の例外は `UnsafeCell` の中にあるメモリです。これは `&` そのものに課される規則で、どこかに `&mut` が
あるかどうかとは関係ありません。そのため、テンプレートの `SAFETY` コメントは的外れです。
`&raw const self.hits as *mut u32` を通して書き込むのは、そのフィールドを指す参照がほかにまったくなくても UB です。
コンパイラはこの規則を前提にしてよく、たとえば関数呼び出しをまたいで `self.hits` をレジスタに置いたままにできます。
Miri のエラーも同じことを指摘しています。`&raw const self.hits` は `&self` からポインタを作るので、そのタグは
「SharedReadOnly（共有の読み取り専用）」の権限しか与えません。`*mut` にキャストしてもそれは変わりません。

解決策は**内部可変性**です。`UnsafeCell` を包み、`&self` 越しに中身を変える安全な API を提供する型を使います。
どれが合うかは、中身をどういう形で取り出す必要があるかで決まります。

- 回数には `Cell<u32>` を使います。`get` は値をコピーして取り出し、`set` は値を置き換えます。
  `Cell` は中身への参照を渡すことがないので、`&self` 越しに中身を変更しても安全です。
- キャッシュする一覧は事情が違います。`keys` は `&self` から借用した普通の `&[String]` を返し、
  呼び出し側はそれを持ったまま `get` や `keys` をまた呼ぶことがあります。`Cell` は `&self` 越しに
  中身を貸すことがそもそもできません。`RefCell` が中身を貸すのはガード越しだけです。
  `self.keys.borrow().as_deref().unwrap()` のスライスは `Ref` から借用していますが、`Ref` は
  `keys` の終わりで破棄される一時値なので、E0515（"cannot return value referencing temporary value"）
  でコンパイルが通りません。これは回避すべき制限ではありません。普通の参照がガードより長生きできたら、
  呼び出し側がまだ読んでいる最中に、別の `&Lookup` からの `borrow_mut` でベクタを置き換えられてしまいます。
- ここで合うのは `OnceCell<Vec<String>>` です。`&self` 越しにできるのは空の状態から値を入れることだけで、
  入っている値を変えたり破棄したりすることはありません。そのため、中身への参照は元の `&self` と同じだけ
  有効なままで、`get_or_init` はその参照を返します。`get_or_init` は、セルが空のときだけクロージャを
  実行して結果を格納し、格納した値への参照（`&Vec<String>`。`&[String]` に型強制されます）を返します。
  クロージャが実行されるのはちょうど一覧を並べ替えるときなので、`sorts` はそこで数えます。
- `insert` は `&mut self` を受け取るので、特別なことは要りません。セルを空の `OnceCell::new()` に
  置き換えます（`self.keys.take()` でも構いません）。その時点で、以前の `keys` が返したスライスが
  使われていないことは、借用チェッカーが保証します。

`get` と `keys` を `&mut self` に変えれば素直に代入できますが、呼び出し側が動かなくなります。
同時に存在する 2 つの `&Lookup` や `Rc<Lookup>` からは共有アクセスしかできませんし、`keys(&mut self)`
では、返したスライスを使っている間はほかのメソッドを呼べません。また、空の一覧を「まだ並べ替えていない」の
意味に使わないよう注意してください。空の表の一覧は空で、その場合も並べ替えは 1 回だけでなければなりません。

`Cell` や `OnceCell` を持つ `Lookup` は `Sync` ではなくなるので、スレッド間では共有できません。
この問題ではそれで構いません。

### 別解

`Lookup` をスレッド間で共有したいなら、スレッドセーフな型を使います。回数には `AtomicU32`、キャッシュには
`OnceLock<Vec<String>>` です。これらも `UnsafeCell` の上に作られています。

```rust
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: AtomicU32,
    misses: AtomicU32,
    keys: OnceLock<Vec<String>>,
    sorts: AtomicU32,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: AtomicU32::new(0),
            misses: AtomicU32::new(0),
            keys: OnceLock::new(),
            sorts: AtomicU32::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        self.keys.take();
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

    pub fn keys(&self) -> &[String] {
        self.keys.get_or_init(|| {
            self.sorts.fetch_add(1, Ordering::Relaxed);
            let mut keys: Vec<String> = self.table.keys().cloned().collect();
            keys.sort_unstable();
            keys
        })
    }

    pub fn sorts(&self) -> u32 {
        self.sorts.load(Ordering::Relaxed)
    }
}
```

`unsafe` を使うなら、キャッシュを `UnsafeCell` の中の `Option<Vec<String>>` のままにしておけます。
`OnceCell` がしていることもおおよそこれと同じです。健全である理由は上と同じで、`&self` 越しにキャッシュへ
書き込むのはそれが `None` のときだけであり、そのときはまだキャッシュを指すスライスが存在しえません。
なお、回数に 1 を足す処理は `Cell::update` を使うと短く書けます。

```rust
use std::cell::{Cell, UnsafeCell};
use std::collections::HashMap;

pub struct Lookup {
    table: HashMap<String, u32>,
    hits: Cell<u32>,
    misses: Cell<u32>,
    keys: UnsafeCell<Option<Vec<String>>>,
    sorts: Cell<u32>,
}

impl Lookup {
    pub fn new(pairs: &[(&str, u32)]) -> Lookup {
        Lookup {
            table: pairs.iter().map(|&(key, value)| (key.to_string(), value)).collect(),
            hits: Cell::new(0),
            misses: Cell::new(0),
            keys: UnsafeCell::new(None),
            sorts: Cell::new(0),
        }
    }

    pub fn insert(&mut self, key: &str, value: u32) {
        self.table.insert(key.to_string(), value);
        *self.keys.get_mut() = None;
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

    pub fn keys(&self) -> &[String] {
        let cache = self.keys.get();
        // SAFETY: through `&self`, the cache is written only while it is `None`, when no slice
        // into it exists. After that it never changes until `insert`, which needs `&mut self`.
        unsafe {
            if (*cache).is_none() {
                let mut keys: Vec<String> = self.table.keys().cloned().collect();
                keys.sort_unstable();
                *cache = Some(keys);
                self.sorts.update(|n| n + 1);
            }
            (*cache).as_deref().unwrap()
        }
    }

    pub fn sorts(&self) -> u32 {
        self.sorts.get()
    }
}
```
