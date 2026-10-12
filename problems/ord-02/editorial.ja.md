`u64::from_str`（`parse::<u64>()` が呼ぶもの）は数字だけかどうかを確かめてはくれません。先頭の `+` を
受け付けるので、`"+1"` や `"1.+2"` が通ってしまいます。まず各部分が ASCII 数字だけでできていることを
確かめ、オーバーフローは `parse` にエラーとして報告させます。同じように、`char::is_alphanumeric` は
`β`、`é`、`٣` などあらゆる Unicode の文字や数字を受け付けます。ASCII だけを認めるのは
`is_ascii_alphanumeric` と `is_ascii_digit` です。

ラベルには `.` が含まれることがあるので（`3-rc.1`）、`.` で分ける**前に**最初の `-` でラベルを
切り離します。`Result<Vec<_>, _>` に `collect` すると最初の不正な部分で止まり、そのあとスライスパターンで
数が 1 ～ 3 個の場合だけを受け付けます。

### 別解

`parse` を使わない場合は、`checked_mul` と `checked_add` で数字を足し上げます。オーバーフローは
パニックではなく `None` になります。`to_digit(10)` は `0` ～ `9` しか受け付けないので、符号も
はじかれます。

```rust
fn parse_number(s: &str) -> Result<u64, ParseVersionError> {
    if s.is_empty() {
        return Err(ParseVersionError);
    }
    s.chars()
        .try_fold(0u64, |n, c| {
            let digit = c.to_digit(10)?;
            n.checked_mul(10)?.checked_add(u64::from(digit))
        })
        .ok_or(ParseVersionError)
}
```
