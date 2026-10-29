`.await` is only allowed directly in the body of an `async fn`, an `async` block or an async closure.
The closure `|f| f.await` is an ordinary closure, even though it is written inside an async function
(E0728): `map` and `sum` call it synchronously, so there is nowhere to suspend.

- Making it an async closure (`async |f| f.await`) does not help: `map` then yields futures, which
  `sum` cannot add up.
- A `for` loop has no closure, so its body is part of the async function. It also awaits the futures
  one at a time, in order.
- Popping them with `while let Some(f) = futs.pop()` (after making `futs` mutable) compiles too, but
  awaits them in reverse order.
