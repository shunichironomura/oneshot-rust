`Drop` を実装した型は分解できません。値に対しては後で `drop` が実行され、`drop` はすべてのフィールドがそろった
`&mut self` を受け取るからです。そのため `let name = self.name;` も、構造体更新構文
`TempFile { name: ..., ..self }` もエラー（E0509）になります。`..self` は、フィールドへのアクセスと同じように
`data` と `log` を `self` からムーブします。`self` を値で受け取っても `Drop` は止まりません。
メソッドの中の `self` はただのローカル変数で、まるごとどこかへムーブしない限り、メソッドの終わりで破棄されます。

これがわかれば `rename` は簡単です。名前をその場で書き換えて `self` をまるごと返せばよく、これは許されています
（引数の `mut` はシグネチャの一部ではありません）。コンパイラはフィールドのクローンを提案しますが、
コンパイルは通っても 2 つ目のファイルができ、元のファイルは `rename` の終わりで破棄されて
`"delete draft (3 bytes)"` が記録されます。

本当の罠は `persist` の `mem::forget(self)` です。`forget` は `drop` を飛ばしますが、**すべてのフィールド**の
デストラクタも飛ばします。`std::mem::take` で `name` と `data` を取り出してから `self` を `forget` すると、
きれいに見えますが、中にはまだ `log` が残っています。その `Rc` は解放されないので、`Rc::strong_count(&log)` は
1 に戻らず、Miri がリークを報告します。`forget` を残したまま安全なコードで取り繕ってもうまくいきません。
`forget` の前に `let log = self.log.clone()` とすると、カウントを 1 つ増やして、後でその 1 つを減らすだけです。
`drop(mem::take(&mut self.log))` は本物のログを解放しますが、代わりに新しい `Rc`（`Rc::default()` が
作ったもの）が入り、それを `forget` がリークさせます。値を `forget` しても問題がないのは、残ったフィールドが
どれも何も所有していないときだけです。

つまり `persist` で `TempFile::drop` を飛ばすなら、ログも含めた**すべての**フィールドに新しい持ち主を
与えなければなりません（フィールドと `Drop` の実装を変えてよいなら、`drop` が確認するフラグで済みますが、
問題文がそれを禁じています）。模範解答は `self` を `ManuallyDrop` で包みます。`ManuallyDrop` は中身を決して破棄しません。
そこから `ptr::read` でフィールドを 1 つずつムーブして取り出すと、どの値も持ち主がちょうど 1 つになります。
`name` と `data` は戻り値になり、`log` は普通に破棄されてログへのハンドルが解放されます。フィールドは
1 つずつ読む必要があります。`ptr::read` で構造体全体をコピーしても、それを分解することはやはりできず
（また E0509 です）、`ManuallyDrop::into_inner` や `ManuallyDrop::drop` を使うと、結局 `TempFile::drop` が
実行されてしまいます。何かを読む前に `self` を包んでおけば、途中でパニックが起きても、二重に破棄される
ものがないことも保証されます。

### 別解

先に `log` を `self` から読み出しておけば、テンプレートの `forget` を残すこともできます。この例では
`ptr::read` と `forget` の間でパニックは起きませんが、もし起きると、巻き戻しで `log` と `self` の両方が
破棄され、同じハンドルが 2 回解放されます。`ManuallyDrop` なら、作りからしてそれが起きません。

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    // SAFETY: この直後に `self` を `forget` するので、
    // このファイルのログへのハンドルを所有するのは `log` だけになる
    let log = unsafe { std::ptr::read(&self.log) };
    std::mem::forget(self);
    drop(log);
    (name, data)
}
```

`drop` は飛ばさない限り必ず実行されるので、`unsafe` を使わずに、`drop` が実行されても害がないように
する方法もあります。`persist` でログを新しい空のログに置き換えます。本物のログはその場で解放され、
`drop` は `delete` の行を使い捨てのログに書き込み、そのログは `self` と一緒に解放されます。
これでも合格しますが、意図がわかりにくく、捨てるためだけにログを 1 つ確保することになります。

```rust
pub fn persist(mut self) -> (String, Vec<u8>) {
    let name = std::mem::take(&mut self.name);
    let data = std::mem::take(&mut self.data);
    self.log = Log::default();
    (name, data)
}
```
