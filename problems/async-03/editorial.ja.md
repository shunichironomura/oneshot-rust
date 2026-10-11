トレイトの中の `async fn get(&self, key: &str) -> Option<u32>` は、
`fn get(&self, key: &str) -> impl Future<Output = Option<u32>>` の略記です。Future の型は実装ごとに
別々で、トレイトはその出力以外について何も約束しません。特に `Send` であることは約束しません。
`total_on_worker` の中でコンパイラが `S` について知っているのは `S: Store` だけなので、`total` の
Future が `await` している間保持する `S::get` の Future を、別のスレッドに移してよいとは
判断できません。`MemStore` のような具体的な型なら、コンパイラは実際の Future を確認できるので
受け入れます。問題がジェネリックなコードでだけ表面化するのはこのためです。

解決策は、メソッドを脱糖した形で書き、境界を加えることです。
`fn get(&self, key: &str) -> impl Future<Output = Option<u32>> + Send;` と書くと、`Send` は
トレイトの約束の一部になります。ジェネリックなコードはそれを前提にでき、各実装はそれを満たすか
検査されます。実装の Future が `Send` でありさえすれば、実装は `async fn` のままでかまわないので、
`MemStore`、`Fallback`、ハーネスのストアは変えずに済みます（`Future` は 2024 エディションから
プレリュードに入っています）。

ただし、これだけでは足りません。Future が `Send` になるのは、`.await` をまたいで保持するものが
すべて `Send` のときだけで、`total` の Future は `s: &S` を保持しています。共有参照 `&S` が
`Send` になるのは `S: Sync` のときだけです。実は、テンプレートに対してコンパイラが最初に報告する
エラーはこちらです。`Fallback` にも同じ問題があります。その `get` は `first` を `await` している間
`&self`（`&Fallback<A, B>`）を保持するので、トレイトが `Send` を要求するようになると、その実装には
`A: Sync` と `B: Sync` が必要です。ハーネスも実装も変えられないので、トレイトの側で保証するしか
ありません。`pub trait Store: Sync` と書くと `Sync` がスーパートレイトになり、`Store` を実装する
型はすべて `Sync` でなければならず、ジェネリックなコードはそれを前提にできます。

次のような書き換えはうまくいきません。

- 2 つの変更のうち片方だけ：もう片方のエラーが残ります。
- `get` にだけ `where Self: Sync` を付ける：`total` や `Fallback` のような呼び出し側が、
  `S: Sync`（や `A: Sync`）を自分で示さなければならなくなります。
- `+ Send + '_` や `+ Send + 'static`：`async fn` の Future は `self` と `key` の両方を借用するので、
  どちらの境界も満たしません（E0477）。ライフタイムの境界は要りません。トレイトの中の
  `impl Trait` の返り値型は、メソッドのすべてのライフタイムをキャプチャできます。
- `Pin<Box<dyn Future<Output = Option<u32>> + Send + '_>>` を返す：`async fn` で書いた実装が
  トレイトと合わなくなります（E0053）。

lint の `async_fn_in_trait` が、公開トレイトの `async fn` に警告を出すのはまさにこの理由からです。
ここでは、トレイトがクレートの外から見えないので警告は出ません。

### 別解

`Send` もスーパートレイトにでき、ライフタイムも明示的に書けます。

```rust
pub trait Store: Send + Sync {
    fn get<'a>(&'a self, key: &'a str) -> impl Future<Output = Option<u32>> + Send + 'a;
}
```

トレイトの `where` 節に書いた `Self` への境界も、スーパートレイトになります。

```rust
pub trait Store
where
    Self: Sync,
{
    fn get(&self, key: &str) -> impl Future<Output = Option<u32>> + Send;
}
```
