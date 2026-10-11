ラッパーのメソッドは、`HashMap::get` そのものと同じ形にします。

```rust
pub fn get<Q>(&self, k: &Q) -> Option<&V>
where
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
```

`K: Borrow<Q>` は「`&K` を `&Q` として見られる」という意味です。`String` は `Borrow<str>` を、
`Vec<T>` は `Borrow<[T]>` を実装していて、どの型も `Borrow<Self>` を実装しているので、`&String`、
`&Vec<u8>`、`&u32` もそのまま使えます。`Borrow` は（`AsRef` と違って）借用した形が元のキーと
まったく同じようにハッシュ・比較されることも約束しているので、マップは `&str` をハッシュして
`String` を見つけられます。`Q` は `impl` ではなく各メソッドの型引数にします。呼び出しごとに
キーの形を選べるようにするためです。`K` への境界には `Q` が出てくるので、メソッドの `where` 節に
書きます。境界の残りの 2 つは忘れやすいものです。

- `Q: ?Sized`：型引数はデフォルトで `Sized` ですが、`str` と `[u8]` は `Sized` ではありません。
  これがないと、`get("apple")` が「the size for values of type `str` cannot be known at
  compilation time」で失敗します。
- `Q: Hash + Eq`：`HashMap::get` は検索に使う値をハッシュし、比較するので、`Q` にこれらの
  トレイトを要求します。`impl` の `K: Hash + Eq` では足りません。

罠は `get_or` です。ライフタイムを省略すると、メソッドの返り値には `&self` のライフタイムが
付きます。つまりテンプレートは「返り値は `self` だけを借用する」と約束していて、`default` を
返すこともある本体は「lifetime may not live long enough」で失敗します。コンパイラが提案するように
`default` と返り値だけに名前付きのライフタイムを付けると、今度は逆に `self.map.get(key)` の
参照のほうが短すぎて失敗します。返り値はどちらから来ることもあるので、`&self`、`default`、
返り値に同じライフタイム `'a` を付けます。レジストリが `default` より長生きしていても問題
ありません。コンパイラは、両方に収まる短さの `'a` を選ぶからです。

キーは `'a` に含めないでください。`key: &'a Q` でも関数単体ではコンパイルが通りますが、返り値が
キーも借用しうると宣言したことになります。すると、一時的な `String` で検索し、返り値を使う前に
それを破棄するハーネスが「temporary value dropped while borrowed」で失敗します。`get`、
`contains`、`remove` にはライフタイムの注釈は要りません。省略規則によって返り値は `&self` に
結び付き、`key` には別のライフタイムが付くので、ちょうど正しい形になります。

### 別解

`Q` だけへの境界は山かっこの中にも書けます。`K: Borrow<Q>` は `K` への境界なので、`where` 節に
残します。`contains` と `remove` も同じように書き換えます。

```rust
pub fn get<Q: Hash + Eq + ?Sized>(&self, key: &Q) -> Option<&V>
where
    K: Borrow<Q>,
{
    self.map.get(key)
}

pub fn get_or<'a, Q: Hash + Eq + ?Sized>(&'a self, key: &Q, default: &'a V) -> &'a V
where
    K: Borrow<Q>,
{
    self.map.get(key).unwrap_or(default)
}
```

`get_or` の `self` に別のライフタイムを付けることもできます。その場合は、それが返り値の
ライフタイムより長生きすることを宣言します。`'s: 'a` は「`'s` は `'a` より長生きする」という
意味で、これによりマップから取り出した `&'s V` を `&'a V` として返せます。

```rust
pub fn get_or<'s, 'a, Q>(&'s self, key: &Q, default: &'a V) -> &'a V
where
    's: 'a,
    K: Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    self.map.get(key).unwrap_or(default)
}
```
