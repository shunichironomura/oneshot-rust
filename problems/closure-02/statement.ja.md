エディタにある 5 つの関数の本体はすでに書かれていますが、コードはコンパイルが通りません。
**シグネチャだけ**（`pub fn` の行と、必要なら `where` 節）を書き換えて、ハーネス（`tests.rs` タブ）が
コンパイルでき、テストに合格するようにしてください。本体は変更しないでください。

- `apply_n(f, n)` は `f` を `n` 回呼びます（`n` が `0` なら 1 回も呼びません）。
- `consume(f)` は `f` を 1 回呼び、その結果を返します。
- `map_pair(f, (x, y))` は `(f(x), f(y))` を返します。先に `f(x)` を呼びます。
- `call_shared(f)` は `f` を 2 回呼び、2 つの結果の和（`u32`）を返します。
- `each_digit(n, f)` は、`n` を 10 進数で書いたときの各桁について `f` を 1 回ずつ呼びます。
  最上位の桁から順に呼び、最後が一の位です。`0` の桁は `0` の 1 つだけです。

ハーネスは、さまざまなクロージャといくつかの普通の関数を渡してこれらを呼び出します。
そのすべての呼び出しがコンパイルできなければなりません。

### 例

```rust
let mut count = 0;
apply_n(|| count += 2, 3);
// count は 6 になる

let name = String::from("Ferris");
consume(move || name)      // String::from("Ferris")
consume(|| vec![1, 2])     // vec![1, 2]

map_pair(|s: &str| format!("{s}!"), ("a", "b"))  // (String::from("a!"), String::from("b!"))
map_pair(|n: i32| n * n, (3, -4))                // (9, 16)

call_shared(|| 2)          // 4
let k = 5;
call_shared(|| k + 1)      // 12

let mut digits = Vec::new();
each_digit(4096, &mut |d| digits.push(d));
// digits は [4, 0, 9, 6] になる
```
