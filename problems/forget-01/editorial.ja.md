`Drop` を実装した型は分解できません。値に対しては後で `drop` が実行され、`drop` はすべてのフィールドがそろった
`&mut self` を受け取るからです。そのため `let name = self.name;` も、構造体更新構文
`TempFile { name: ..., ..self }` もエラー（E0509）になります。`..self` は、フィールドへのアクセスと同じように
`data` と `log` を `self` からムーブします。`self` を値で受け取っても `Drop` は止まりません。
メソッドの中の `self` はただのローカル変数で、まるごとどこかへムーブしない限り、メソッドの終わりで破棄されます。

コンパイラはクローンを提案しますが、コンパイルは通っても正しくありません。`rename` でフィールドをクローンすると
2 つ目のファイルができ、元のファイルは `rename` の終わりで破棄されて `"delete draft (3 bytes)"` が記録されます。
`persist` でクローンすると、元のフィールドは `forget` した値の中に残ったままリークします。

本当の罠は `mem::forget(self)` です。`forget` は `drop` を飛ばしますが、**すべてのフィールド**のデストラクタも
飛ばします。`std::mem::take` で `name` と `data` を取り出してから（`mut self` が必要です）`self` を `forget` すると、
きれいに見えますが、中にはまだ `log` が残っています。その `Rc` は解放されないので、`Rc::strong_count(&log)` は
1 に戻らず、Miri がリークを報告します。その場しのぎの修正もうまくいきません。`forget` の前に `let log = self.log.clone()` と
すると、カウントを 1 つ増やして、後でその 1 つを減らすだけです。`drop(mem::take(&mut self.log))` は本物のログを
解放しますが、代わりに新しい `Rc`（`Rc::default()` が作ったもの）が入り、それを `forget` がリークさせます。
値を `forget` しても問題がないのは、残ったフィールドがどれも何も所有していないときだけです。

模範解答は `drop` をまったく飛ばしません。`persisted` フラグを追加し、`persist` はそれを立てて名前と内容を
取り出します。`self` はメソッドの終わりで普通に破棄され（このとき `log` も解放されます）、`drop` は永続化された
ファイルについては何も記録しません。これは手作りのドロップフラグです。*変数*がムーブされたかどうかは
コンパイラが自分で追跡し、ムーブされていなければ破棄しますが、`drop` にまだ仕事が残っているかどうかを
知っているのは型だけです。`rename` にはフラグも要りません。名前をその場で書き換えて `self` をまるごと返せば
よく、これは許されています。禁止されているのは、フィールドを取り出すことだけです。引数の `mut` は
シグネチャの一部ではありません。

### 別解

テンプレートの意図どおりに `drop` を飛ばすなら、ファイルを `ManuallyDrop` で包み、`log` も含めて
**すべての**フィールドを `ptr::read` で取り出します。`name` と `data` だけを読むと、`forget` のときと
同じようにログがリークします。また、`ManuallyDrop::drop` や `ManuallyDrop::into_inner` を使うと、結局
`TempFile::drop` が実行されてしまいます。`rename` は模範解答と同じで、`TempFile` のフィールドは 3 つのままです。

```rust
use std::mem::ManuallyDrop;
use std::ptr;

// `impl TempFile` の中
pub fn persist(self) -> (String, Vec<u8>) {
    // Neither `drop` nor the destructors of the fields run on `file`.
    let file = ManuallyDrop::new(self);
    // SAFETY: each field is read exactly once and `file` is never used again,
    // so nothing is dropped twice.
    let (name, data, log) =
        unsafe { (ptr::read(&file.name), ptr::read(&file.data), ptr::read(&file.log)) };
    drop(log);
    (name, data)
}
```

`drop` は飛ばさない限り必ず実行されるので、`drop` が実行されても害がないようにする方法もあります。
元の 3 つのフィールドのまま（`rename` は模範解答と同じ）、`persist` でログを新しい空のログに置き換えます。
本物のログはその場で解放され、`drop` は `delete` の行を使い捨てのログに書き込み、そのログは `self` と一緒に
解放されます。これでも合格しますが、意図がわかりにくく、捨てるためだけにログを 1 つ確保することになります。

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    self.log = Log::default();
    (name, data)
}
```
