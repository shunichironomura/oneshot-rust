`HashSet` や `HashMap` は 2 段階でキーを探します。ハッシュ値でどこを探すかを決め、そこで見つかった
要素を `==` で確かめます。これがうまくいくには**等しいキーのハッシュ値が等しい**必要があり、それが
`Hash` の約束です（`a == b` なら `hash(a) == hash(b)` でなければなりません）。`eq` が大文字・小文字を
無視するなら、`hash` も無視しなければなりません。

つい書きたくなる `self.0.hash(state)`（`#[derive(Hash)]` が生成するものと同じ）では、`Rust` と `RUST` の
ハッシュ値が違います。すると `contains` はたいてい違う場所を探して `false` を返し、`unique_tags` は
両方のつづりを残してしまいます。これは UB ではなく論理エラーです。クラッシュはせず、結果が間違う
だけで、たまたまハッシュ値の一部が衝突して探索が成功することさえあります。

そこで、`eq` が比べるものをそのままハッシュします。つまり、ASCII の英字を小文字にしたバイト列です。
`eq_ignore_ascii_case` や `to_ascii_lowercase` が変えるのは `A`–`Z` だけで、規則のとおりです。
`to_lowercase` や `to_uppercase` は Unicode の規則に従うので、これらで比べると `É` と `é` が等しく
なり、`to_uppercase` では `Straße` と `STRASSE` まで等しくなります。最後の `0xff` は `str` のやり方を
まねたもので、文字列の終わりを表します。これで `("ab", "c")` と `("a", "bc")` の組のハッシュ値が
違うものになります（テストには必要ありません）。等しくないタグのハッシュ値が同じになるのは
構いません。ただの衝突です。

最初のつづりを残すのに特別なコードは要りません。`HashSet::insert` は、等しい値がすでにあれば
`false` を返し、入っている値はそのままにします。`HashMap::entry` も（`HashMap::insert` と同じく）
既存のキーを残し、新しいキーを捨てます。`set.get(&Tag::new("rust"))` が入っている `Rust` を返すのも
同じ理由です。`Display` は、その保存されたつづりを `f.write_str(&self.0)` や `write!(f, "{}", self.0)` で
書き出すだけです。

`Borrow` のドキュメントにも似た `CaseInsensitiveString` の例があり、このような型に `Borrow<str>` を
実装してはいけない理由が説明されています。`str` は大文字・小文字を区別して比べ、ハッシュするので、
`map.get("rust")` で探すとキーとの比較が食い違ってしまいます。

### 別解

小文字にしたコピーをハッシュするのが、いちばん短い正しい `hash` です。ただし呼ぶたびにメモリを
確保します。

```rust
impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_ascii_lowercase().hash(state);
    }
}
```

`eq` も同じように小文字にしたコピーどうしを比べられますが、`eq_ignore_ascii_case` ならメモリを確保せずに
比べられます。

`unique_tags` は、それまでに集めたタグと `==` で比べれば、ハッシュを使わなくても書けます。
時間が 2 乗に比例するので、長いリストには `HashSet` を使う版のほうが向いています。

```rust
pub fn unique_tags(words: &[&str]) -> Vec<Tag> {
    let mut tags: Vec<Tag> = Vec::new();
    for &word in words {
        let tag = Tag::new(word);
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }
    tags
}
```
