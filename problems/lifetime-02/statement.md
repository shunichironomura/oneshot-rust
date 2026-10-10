`Tokenizer` borrows an input string and yields its whitespace-separated tokens one by one. The logic
is complete, but **lifetime annotations** are missing, so it does not compile.

Add lifetimes to the types and signatures so that the test harness (the `tests.rs` tab) can use it.
Do not change the method bodies.

The harness keeps the tokens returned by `next_token` in a `Vec` while it keeps calling `next_token`.
What should the returned reference's lifetime be tied to?
