キャプチャした変数を書き換えるクロージャは、その変数を可変で借用します。この借用は、呼び出している間だけではなく、
クロージャがまだ呼ばれる可能性がある間、つまり**最後に使われるまで**続きます。ループの中の `record` は次の周でも
呼ばれるので、ループ全体にわたって `&mut errors` と `&mut log` を持ち続けます。そのため、2 回の呼び出しの間で
`errors` を読むとエラーになります（E0503「cannot use `errors` because it was mutably borrowed」）。

ループに必要な情報をクロージャから渡すようにすれば直ります。ここでは、イベントを記録した後のエラーの数を返します。
ループの中の `log.push("abort")` はそのままで構いません。借用チェッカーは制御の流れを追います。この `push` の直後は
`break` なので、その経路では `record` が二度と呼ばれず、`record` による `log` の借用はもう終わっています。
同じ理由で、ループの後の `(errors, log)` も問題ありません。

`record` に `move` を付けても解決しません。`errors` は `Copy` なので、クロージャは自分のコピーを数えるだけで、
ループから見える値は `0` のままです。`log` はクロージャの中にムーブされ、ループからは使えなくなります。

`make_greeter` には問題が 2 つあります。返したクロージャは `make_greeter` から戻った後に使われるので、
関数が所有しているもの（引数も含みます）を借用してはいけません。`move` が必要です。
ただし、`move` だけでは足りません。`let greeting = greeting.trim();` の後の `greeting` は、
引数の `String` を借用する `&str` を指しています。この `String` は関数から戻るときに破棄されます。
その参照をクロージャにムーブしても、クロージャは `String` を借用したままです
（E0597「`greeting` does not live long enough」）。`to_string()` で、空白を取り除いた文字列を
所有する `String` にしてから、それをムーブします。

### 別解

クロージャの本体をループの中に直接書く方法です。クロージャがなければ、処理の合間に借用を持ち続けるものはありません。

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>) {
    let mut errors = 0;
    let mut log = Vec::new();
    for event in events {
        if event.starts_with("ERR") {
            errors += 1;
        }
        log.push(event.to_string());
        if errors == limit {
            log.push("abort".to_string());
            break;
        }
    }
    (errors, log)
}
```

状態を引数で渡す方法もあります。何もキャプチャしないクロージャは借用を持たず、`errors` と `log` を借用するのは
各呼び出しの実行中だけです。

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>) {
    let mut errors = 0;
    let mut log = Vec::new();
    let record = |errors: &mut usize, log: &mut Vec<String>, event: &str| {
        if event.starts_with("ERR") {
            *errors += 1;
        }
        log.push(event.to_string());
    };
    for event in events {
        record(&mut errors, &mut log, event);
        if errors == limit {
            log.push("abort".to_string());
            break;
        }
    }
    (errors, log)
}
```

あいさつの関数は、`String` をまるごとムーブして、呼ぶたびに空白を取り除くこともできます。

```rust
pub fn make_greeter(greeting: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {name}!", greeting.trim())
}
```
