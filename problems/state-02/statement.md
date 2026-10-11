The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

A `Post` is a blog post that has to be reviewed before it is published. It keeps its current state as
a trait object (`Option<Box<dyn State>>`), and the state structs `Draft`, `PendingReview` and
`Published` decide what each call does.

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

- `new` creates a draft with empty text.
- `add_text` appends `text` to the post's text while the post is a draft, and does nothing in any
  other state.
- `request_review` turns a draft into a post pending review.
- `approve` approves a post pending review. The **second** approval publishes it.
- `reject` sends a post pending review back to the draft state. Its text is kept, but its approval
  is discarded: after the next `request_review`, it needs two approvals again.
- `content` returns the text of a published post, and `""` in any other state.
- Every other call does nothing. In particular, `request_review` on a post pending review keeps its
  approval, and a published post stays published.

The `State` trait and the state structs are correct. Do not change them or the public signatures,
and keep `Post` delegating to its state object.

### Examples

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
post.reject();          // back to a draft; the approval is discarded
post.add_text("a salad");
post.request_review();
post.approve();
post.content() // ""
post.approve();
post.content() // "I ate a salad"
```
