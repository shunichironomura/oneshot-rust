次のコードはコンパイルが通りません。**エラーメッセージを見ずに**、どこで怒られるかを予想して直してください。

`Post` はブログの記事で、公開する前にレビューを受ける必要があります。記事は今の状態をトレイトオブジェクト
（`Option<Box<dyn State>>`）として持ち、各メソッドの動作は状態を表す構造体 `Draft`、`PendingReview`、
`Published` が決めます。

```rust
impl Post {
    pub fn new() -> Post;
    pub fn add_text(&mut self, text: &str);
    pub fn content(&self) -> &str;
    pub fn request_review(&mut self);
    pub fn approve(&mut self);
    pub fn reject(&mut self);
}
```

- `new` は本文が空の下書きを作ります。
- `add_text` は、記事が下書きのときは本文の末尾に `text` を追加し、それ以外の状態では何もしません。
- `request_review` は下書きをレビュー待ちにします。
- `approve` はレビュー待ちの記事を承認します。**2 回目**の承認で記事は公開されます。
- `reject` はレビュー待ちの記事を下書きに戻します。本文はそのまま残りますが、承認は取り消されます。
  次に `request_review` した後は、もう一度 2 回の承認が必要です。
- `content` は、公開された記事なら本文を、それ以外の状態なら `""` を返します。
- これ以外の呼び出しは何もしません。特に、レビュー待ちの記事に `request_review` しても承認はそのまま残り、
  公開された記事はずっと公開されたままです。

公開されているシグネチャと状態の構造体は変えないでください。
また、`Post` が状態オブジェクトに処理を任せる作りは残してください。

### 例

```rust
let mut post = Post::new();
post.add_text("salad");
post.request_review();
post.approve();
post.content() // ""
post.approve();
post.content() // "salad"
```

```rust
let mut post = Post::new();
post.add_text("I ate ");
post.request_review();
post.approve();
post.reject();          // 下書きに戻り、承認は取り消される
post.add_text("a salad");
post.request_review();
post.approve();
post.content() // ""
post.approve();
post.content() // "I ate a salad"
```
