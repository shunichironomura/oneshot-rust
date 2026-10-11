A *lender* hands out items one at a time, like an iterator. The two lenders below hand out items
that borrow from the lender itself:

- `windows_mut(slice, size)` yields the windows of `size` consecutive elements of `slice` as
  `&mut [T]`, from left to right. Each window starts one element after the previous one, so
  neighboring windows overlap.
- `numbered(text)` yields each line of `text` with its line number in front, such as
  `"1: first line"`, as a `&str`. It writes every line into the same `String` buffer, which it
  reuses.

The bodies are already written, but the declarations are wrong and the code does not compile.
Change **only the declarations**: the trait `Lender`, and in its two impls, the `type Item` line and
the signature of `next`. The test harness (the `tests.rs` tab) must compile and pass. Do not change
the bodies, the structs, or the functions `windows_mut`, `numbered` and `count_all`.

```rust
pub fn windows_mut<T>(slice: &mut [T], size: usize) -> WindowsMut<'_, T>
pub fn numbered(text: &str) -> Numbered<'_>
pub fn count_all<L: Lender>(mut lender: L) -> usize
```

- `size` must be at least 1 (`windows_mut` panics on 0). If `size` is larger than the length of
  `slice`, there are no windows. The windows share the elements of `slice`, so a change made
  through one window is seen by the windows after it. `T` can be any type.
- `numbered` splits `text` into lines as `str::lines` does: a line ends at `\n` or `\r\n`, and a
  line ending at the very end of `text` does not start another line. Lines are numbered from 1, and
  an empty line gets its number too, as in `"2: "`. An empty text has no lines.
- Once a lender has returned `None`, it keeps returning `None`.
- `count_all` calls `next` until it returns `None` and returns how many items it got. Items taken
  before the call are not counted.
- The items of two different lenders can be used at the same time.

### Examples

```rust
let mut v = vec![1, 2, 3, 4];
let mut windows = windows_mut(&mut v, 2);
while let Some(w) = windows.next() {
    w[1] += w[0];
}
// v is now [1, 3, 6, 10]

let mut lines = numbered("apple\nbanana");
lines.next()  // Some("1: apple")
lines.next()  // Some("2: banana")
lines.next()  // None

count_all(windows_mut(&mut [0; 5], 3))  // 3
count_all(numbered("a\nb\nc"))          // 3
```
