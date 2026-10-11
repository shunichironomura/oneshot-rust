次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`TempFile` はメモリ上の一時ファイルです。破棄されると削除されますが、永続化（persist）したファイルは
削除されません。各ファイルは、自分に起きたことを共有のログに書き込みます。

```rust
pub type Log = Rc<RefCell<Vec<String>>>;

impl TempFile {
    pub fn create(name: &str, log: &Log) -> TempFile;
    pub fn write(&mut self, bytes: &[u8]);
    pub fn rename(self, new_name: &str) -> TempFile;
    pub fn persist(self) -> (String, Vec<u8>);
}

impl Drop for TempFile { /* ... */ }
```

- `create` は、`log` に書き込む `name` という名前の空のファイルを作り、ログに `"create <name>"` を追加します。
  名前はどんな文字列でもよく、空文字列のこともあります。
- `write` はファイルの内容の末尾に `bytes` を追加します。ログには何も書きません。
- `rename` はログに `"rename <old> -> <new>"`（`<old>` は今の名前、`<new>` は `new_name`）を追加し、
  名前を `new_name` に変えたファイルを返します。内容はそのままです。名前を変えても何も削除されません。
  ログに追加されるのは `rename` の行だけで、名前を変えたファイルを後で破棄すると、新しい名前で削除されます。
- `persist` はファイルを消費して、その名前と内容を返します。永続化したファイルは決して削除されません。
  ログには、そのときも後でも何も追加されません。
- 永続化していないファイルを破棄すると、ファイルは削除され、ログに `"delete <name> (<n> bytes)"` が
  追加されます。`<n>` は内容のバイト数です（1 バイトのときも `"(1 bytes)"` です）。
- なくなったファイル（破棄したファイルと永続化したファイル）が、ログを生かし続けてはいけません。
  `log` に書き込むファイルが 1 つも残っていなければ、`Rc::strong_count(&log)` は `1` に戻ります。
  メモリリークは Miri が検出し、不正解になります。

`Log` と公開されているシグネチャは変えないでください。`TempFile` の非公開のフィールドは変更して構いません。
`unsafe` のコードを使っても構いません。

### 例

```rust
let log: Log = Rc::new(RefCell::new(Vec::new()));
let mut a = TempFile::create("a", &log);
a.write(b"hi");
let (name, data) = a.persist(); // name == "a"、data == b"hi"
// Rc::strong_count(&log) == 1
let b = TempFile::create("b", &log);
drop(b);
// log: ["create a", "create b", "delete b (0 bytes)"]
```

```rust
let mut f = TempFile::create("draft", &log); // "create draft" を追加
f.write(b"abc");
let f = f.rename("final");                   // "rename draft -> final" を追加
drop(f);                                     // "delete final (3 bytes)" を追加
```
