`for` ループのパターンは、ふつうの `let` のパターンと同じく**論駁不可能**（irrefutable、どんな値にも
必ずマッチする）でなければなりません（E0005）。`(i, Some(x))` は `None` の枠にマッチしませんが、
`for` ループは要素を読み飛ばさないので、`None` が来たときにどうするかをコンパイラは決められません。
枠全体を変数に束縛し、本体の `if let` で調べます。`slot` は `&Option<u8>` なので、`Some(x)` は `x` を
`&u8` として束縛し、`*x` で値をコピーして取り出します。

関数やクロージャの引数も `let` や `for` と同じ規則で、パターンは論駁不可能でなければなりません。
論駁可能なパターンを書く場所は、`match`、`if let`、`while let`、let-else です。

### 別解

let-else と `continue` で空の枠を読み飛ばす方法です。この文より後では `x` に値が入っていなければ
ならないので、`else` ブロックは `return`、`break`、`continue` やパニックで、必ず外へ抜け出さなければ
なりません。

```rust
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    let mut filled = Vec::new();
    for (i, slot) in slots.iter().enumerate() {
        let Some(x) = slot else { continue };
        filled.push((i, *x));
    }
    filled
}
```

イテレータアダプタを使うなら、`filter_map` で調べ、`Option::map` で組を作ります。位置が空の枠も数えるように、
絞り込む前に `enumerate` します。

```rust
pub fn filled_slots(slots: &[Option<u8>]) -> Vec<(usize, u8)> {
    slots
        .iter()
        .enumerate()
        .filter_map(|(i, slot)| slot.map(|x| (i, x)))
        .collect()
}
```
