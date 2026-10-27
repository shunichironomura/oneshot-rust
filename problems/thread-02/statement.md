The code below does not compile. Fix it.

`letter_counts` counts the chars in a list of words, splitting the work across several threads.

```rust
pub fn letter_counts(words: Vec<String>, workers: usize) -> (HashMap<char, usize>, usize)
```

- `workers` is at least 1. Worker `w` (`0 <= w < workers`) handles the words at indices `w`,
  `w + workers`, `w + 2 * workers`, ... on a thread of its own, started with `thread::spawn`. A worker
  may get no words at all, and `words` may be empty.
- For each of its words, a worker adds every char of the word to one map shared by all workers, and
  adds 1 to one counter shared by all workers. Empty words count as processed words too.
- Every `char` is counted as it is: uppercase and lowercase letters are different chars, and spaces,
  punctuation and non-ASCII chars such as `é` are counted too.
- Once every thread has been joined, the function returns the map (each char that occurs, with its
  number of occurrences in all the words) and the counter (the number of words processed). Chars that
  do not occur are not in the map.

Do not change the signature, and keep starting one thread per worker with `thread::spawn`.

### Example

`letter_counts(vec!["ab".into(), "b".into(), "".into()], 2)` returns `({'a': 1, 'b': 2}, 3)`.
Worker 0 handles `"ab"` and `""`, and worker 1 handles `"b"`.
