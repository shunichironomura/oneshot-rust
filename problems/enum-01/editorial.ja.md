`match` はすべてのバリアントを網羅する必要があるので、書き忘れはバグではなくコンパイルエラーになります。
アームは上から順に試されます。`Light::Flashing(0)` は `Light::Flashing(n)` より前に書かないと、`0` のときに
`n - 1` がアンダーフローします（デバッグビルドではパニック）。数を使わないときは `Light::Flashing(_)` で
どの回数にもマッチできます。

`Light` は `Copy` を derive しているので、メソッドは `self` を値で受け取れます。`light = light.next()` は古い値を
置き換えるだけです。`time_until_green` は `Green` になるまで秒数を足していきます。どの状態からも最後は
`Green` になるので、ループは終わります。

### 別解

マッチガードを使っても、回数が 0 かどうかで分けられます。

```rust
pub fn next(self) -> Light {
    match self {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
        Light::Flashing(n) if n > 0 => Light::Flashing(n - 1),
        Light::Flashing(_) => Light::Red,
    }
}
```

`time_until_green` は再帰でも書けます。

```rust
pub fn time_until_green(self) -> u32 {
    match self {
        Light::Green => 0,
        other => other.duration_secs() + other.next().time_until_green(),
    }
}
```
