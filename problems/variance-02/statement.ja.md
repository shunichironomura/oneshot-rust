次のコードはコンパイルが通りません。直してください。

`unique_upper` は、テキストに含まれる単語を重複なしで大文字にして並べます。

```rust
pub fn unique_upper(text: &str) -> Vec<String>
```

- `text` は空白で単語に分けます。`split_whitespace` と同じく、連続した空白文字（スペース、タブ、
  改行など）は 1 つの区切りとして扱います。
- 各単語は最初に出てきたものだけを残し、最初に出てきた順に返します。単語は大文字にする前の、
  `text` に書かれたままの形で比較するので、`"a A"` は `["A", "A"]` になります。
- 残した単語は `to_uppercase` で大文字にします。Unicode の規則に従うので、`"straße"` は
  `"STRASSE"` になります。

`unique_upper` のシグネチャは変えないでください。非公開のヘルパーは変更・削除して構いません。

### 例

- `unique_upper("to be or not to be")` は `["TO", "BE", "OR", "NOT"]` を返します。
