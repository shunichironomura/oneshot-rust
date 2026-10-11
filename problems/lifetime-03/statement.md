`matching_lines` lazily yields the lines of `text` that contain `pattern`, ignoring case. The body is
complete, but the signature does not compile.

Change **only the signature** so that the test harness (the `tests.rs` tab) can use it. Do not
change the body.

The harness builds the pattern in a temporary `String`, which is dropped **before** the returned
iterator is consumed. Only the lines of `text` are kept.

### Example

For `text = "Rust is fast\nPython is easy\nI trust rustc"` and the pattern `"rust"`, the iterator
yields `"Rust is fast"` and `"I trust rustc"`.
