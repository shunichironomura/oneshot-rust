`matching_lines` は、`text` の行のうち `pattern` を含むもの（大文字・小文字は区別しない）を遅延して返します。
本体は完成していますが、シグネチャがコンパイルを通りません。

ハーネス（`tests.rs` タブ）から使えるように、**シグネチャだけ**を書き換えてください。本体は変更しないでください。

ハーネスはパターンを一時的な `String` で作り、返されたイテレータを読み進める**前に**その `String` を破棄します。
手元に残るのは `text` の行だけです。

### 例

`text = "Rust is fast\nPython is easy\nI trust rustc"`、パターン `"rust"` に対して、
イテレータは `"Rust is fast"` と `"I trust rustc"` を返します。
