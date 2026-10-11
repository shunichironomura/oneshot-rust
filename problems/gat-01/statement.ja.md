*レンダー*（lender）は、イテレータのように要素を 1 つずつ渡していくものです。次の 2 つのレンダーが渡す要素は、
レンダー自身を借用しています。

- `windows_mut(slice, size)` は、`slice` の連続した `size` 個の要素からなる窓を、左から順に `&mut [T]` として
  返します。各窓は直前の窓より 1 要素後ろから始まるので、隣り合う窓は重なります。
- `numbered(text)` は、`text` の各行の先頭に行番号を付けたもの（`"1: first line"` など）を `&str` として
  返します。どの行も同じ `String` のバッファに書き込み、そのバッファを使い回します。

本体はすでに書かれていますが、宣言が間違っていてコンパイルが通りません。**宣言だけ**を書き換えてください。
書き換えてよいのは、トレイト `Lender` と、2 つの impl の中の `type Item` の行と `next` のシグネチャです。
テストハーネス（`tests.rs` タブ）がコンパイルでき、テストに合格するようにしてください。本体、構造体、
関数 `windows_mut`・`numbered`・`count_all` は変更しないでください。

```rust
pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T>
pub fn numbered(text: &str) -> Numbered<'_>
pub fn count_all<L: Lender>(mut lender: L) -> usize
```

- `size` は 1 以上でなければなりません（`windows_mut` は 0 でパニックします）。`size` が `slice` の長さより
  大きければ、窓は 1 つもありません。窓どうしは `slice` の要素を共有しているので、ある窓を通して
  変更した値は、その後の窓から見えます。`T` はどんな型でも構いません。
- `numbered` は `str::lines` と同じように `text` を行に分けます。行は `\n` か `\r\n` で終わり、`text` の
  最後にある改行の後には新しい行はありません。行番号は 1 から始まり、空行にも `"2: "` のように番号が付きます。
  空のテキストには行がありません。
- レンダーは一度 `None` を返したら、その後もずっと `None` を返します。
- `count_all` は `None` が返るまで `next` を呼び、得られた要素の個数を返します。呼び出す前に取り出した要素は
  数えません。
- 別々のレンダーの要素は、同時に使えます。

### 例

```rust
let mut v = vec![1, 2, 3, 4];
let mut windows = windows_mut(&mut v, 2);
while let Some(w) = windows.next() {
    w[1] += w[0];
}
// v は [1, 3, 6, 10] になる

let mut lines = numbered("apple\nbanana");
lines.next()  // Some("1: apple")
lines.next()  // Some("2: banana")
lines.next()  // None

count_all(windows_mut(&mut [0; 5], 3))  // 3
count_all(numbered("a\nb\nc"))          // 3
```
