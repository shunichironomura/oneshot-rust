エラーは `first.to_ascii_lowercase() - 'a'` です。`char` にはそもそも算術演算がなく、`char` どうしの
引き算もできません。Rust が `char` を自動で整数に変換することもありません。先に整数に変換します。
ここでの `first` は ASCII の英字なので `as u8` で情報は失われません。`b'a'` はバイト `97` です。
`(first.to_ascii_lowercase() as u8 - b'a') as usize` と書きます。

`Default::default()` はこのままで問題ありません。配列は要素の型が `Default` なら（長さ 32 までは）
`Default` を実装しているので、別々の空の `Vec` が 26 個できます。

算術演算の周りのコードはそのまま残してください。`to_ascii_lowercase` がないと、
`b'A' - b'a'` がアンダーフローし、デバッグビルドではパニックします。`is_ascii_alphabetic` を
`is_alphabetic` にすると、ASCII 以外の英字もチェックを通り、`as u8` はコードポイントの下位 1 バイトしか
残しません。`ñ`（U+00F1）では添字が 25 をはるかに超え、`š`（U+0161）にいたってはバケツ `a` に
入ってしまいます。

### 別解

`first.to_ascii_lowercase() as usize - 'a' as usize` でも構いません。`char` はどの整数型にも
キャストできます。制限があるのは `char` *への*キャストだけで、`u8` に限られます。
