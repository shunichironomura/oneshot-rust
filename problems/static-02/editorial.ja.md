Rust には型ごとの static はないので、カウンタは型から回数への**1 つの** `static` なマップに入れます。
型の識別には `TypeId::of::<T>()` を使います。どの関数にも `T: 'static` が付いているのはこのためです。
ライフタイムはプログラムの実行前に消えてしまうので、`&'a str` と `&'static str` に別々の ID を付けることは
できません。

- static の初期化式はコンパイル時に評価されるので、呼べるのは `const fn` だけです。`Mutex::new` と
  `BTreeMap::new` は `const fn` で、`TypeId` は `Ord` を実装しているので、`Mutex<BTreeMap<TypeId, usize>>` なら
  遅延初期化は要りません。`HashMap::new` は `const` ではありません（E0015）。別解を見てください。
- static はどのスレッドからも触れるので、`Sync` でなければなりません。`RefCell` ではなく `Mutex` を使います。
- `register` は 1 回のロックの中で値を増やして読み出すので、2 つのスレッドが同じ値を受け取ることは
  ありません。`count` は `entry(..).or_insert(0)` ではなく `get` を使います。こうすれば、型について
  問い合わせただけでマップに追加されることはありません（追加されると `distinct` に数えられてしまいます）。

**落とし穴**は、ジェネリック関数の中で宣言した static です。`T` ごとに 1 つのカウンタがあるように見えます。

```rust
fn counter<T: 'static>() -> &'static AtomicUsize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    &COUNT
}
```

これはコンパイルできますが、`COUNT` は 1 つしかありません。コンパイラは関数のコードを `T` ごとに複製
（単相化）しますが、static は複製しません。ジェネリック関数の中の static は、関数の外で宣言したのと
同じく 1 つのアイテムです（`T` を使うことさえできず、使うとエラー E0401 になります）。すべての型が
1 つのカウンタを共有するので、最初の例の `register::<Pear>()` は `3` を返してしまいます。C++ では
関数テンプレートの `static` ローカル変数はインスタンス化ごとに別々なので、ここが違います。

ほかに 2 つの近道も、非公開のテストケースで失敗します。

- `std::any::type_name::<T>()` は一意なキーになりません。2 つのテストケースでそれぞれ宣言した
  `Apple` という構造体は別の型ですが、名前はどちらも `main::tests::run::{{closure}}::Apple` です。
  ドキュメントにも、この名前を型の一意な識別子とみなしてはいけないと書かれています。
- `thread_local!` はコンパイルできますが、スレッドごとに別々のカウンタになってしまいます。

### 別解

`HashMap` でも書けます。`HashMap::new` は `const` ではない（`RandomState` が実行時にランダムな鍵を
選ぶ）ので、ここでは `LazyLock` を使って、最初に使うときにマップを作ります。4 つの関数は模範解答と
同じです。

```rust
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static COUNTS: LazyLock<Mutex<HashMap<TypeId, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
```

ただし `HashMap::with_hasher` は `const` なので、ランダムな鍵を必要としないハッシャーを使えば、
遅延初期化も要りません（4 つの関数はやはり同じです）。

```rust
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, DefaultHasher};
use std::sync::Mutex;

type Counts = HashMap<TypeId, usize, BuildHasherDefault<DefaultHasher>>;

static COUNTS: Mutex<Counts> = Mutex::new(HashMap::with_hasher(BuildHasherDefault::new()));
```

static には構造体も入れられますし、構造体の式は `const` な初期化式の中に書けます。ここでは合計をマップの
隣に置いて同じロックで守るので、`total` はすべてのカウンタを足し合わせなくて済みます。

```rust
use std::any::TypeId;
use std::collections::BTreeMap;
use std::sync::Mutex;

struct Counters {
    per_type: BTreeMap<TypeId, usize>,
    total: usize,
}

static COUNTERS: Mutex<Counters> = Mutex::new(Counters {
    per_type: BTreeMap::new(),
    total: 0,
});

pub fn register<T: 'static>() -> usize {
    let mut counters = COUNTERS.lock().unwrap();
    counters.total += 1;
    let n = counters.per_type.entry(TypeId::of::<T>()).or_insert(0);
    *n += 1;
    *n
}

pub fn count<T: 'static>() -> usize {
    let counters = COUNTERS.lock().unwrap();
    counters.per_type.get(&TypeId::of::<T>()).copied().unwrap_or(0)
}

pub fn total() -> usize {
    COUNTERS.lock().unwrap().total
}

pub fn distinct() -> usize {
    COUNTERS.lock().unwrap().per_type.len()
}
```
