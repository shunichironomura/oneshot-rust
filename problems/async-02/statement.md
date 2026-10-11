The bodies of `retry` and `fetch_any` are already written, but their signatures are wrong and the
code does not compile. Change **only the signatures** so that the test harness (the `tests.rs` tab)
can call both functions. Do not change the bodies.

- `retry(times, f)` calls `f` and awaits the future it returns, over and over, until a result is
  `Ok` or `f` has been called `times` times. It resolves to the last result. `times` is at least 1.
- `fetch_any(mirrors, path, fetch)` uses `retry` to call `fetch` with the URL `{mirror}/{path}` of
  each mirror in order, until a result is `Ok`. It resolves to the last result. `mirrors` is not
  empty.

The harness runs the futures with its own small executor (only `std` is available). Most of the
callbacks it passes are async closures (`async || ...` and `async |url: &str| ...`) that update
variables they capture, such as a counter or a log; the harness reads those variables after the
call. It also passes the async function `probe` to `fetch_any`.

### Examples

- Take a closure that counts its calls in `calls` and returns `Err("busy")` on the first two calls
  and `Ok(calls)` from the third call on. `retry(5, ...)` resolves to `Ok(3)` and leaves `calls`
  at `3`, while `retry(2, ...)` resolves to `Err("busy")` and leaves `calls` at `2`.
- `fetch_any(&["a", "b", "c"], "x.txt", ...)` with a closure that succeeds only for mirror `b`
  calls it with `"a/x.txt"`, then with `"b/x.txt"`, and resolves to the result of the second call.
