The code below does not compile. **Without looking at the error message**, predict where the compiler
complains and fix it.

The two functions below work with 26 buckets of words, one for each letter of the English
alphabet.

```rust
pub fn bucket_by_initial(words: &[&str]) -> [Vec<String>; 26]
pub fn letters_used(buckets: &[Vec<String>; 26]) -> String
```

- Bucket `0` is for `a`, bucket `1` for `b`, and so on up to bucket `25` for `z`.
- `bucket_by_initial` puts each word whose first character is an ASCII letter into the bucket of that
  letter. Upper and lower case count as the same letter: `"apple"` and `"Avocado"` both go to bucket
  `0`. The words are stored as they are (not lowercased), and each bucket keeps them in the order in
  which they appear in `words`, duplicates included.
- Every other word is ignored: the empty string, and every word whose first character is not an
  ASCII letter, such as a digit, a space, a punctuation mark or any non-ASCII character. Characters
  outside ASCII never count as letters here, even letters such as `é`.
- `letters_used` returns the lowercase letters whose buckets are not empty, in alphabetical order. It
  only looks at whether each bucket is empty, not at the strings inside. If every bucket is empty, it
  returns `""`.

Do not change the signatures.

### Examples

- `bucket_by_initial(&["bob", "Al", "42", "éclair", "", "beth"])` returns an array whose bucket `0`
  is `["Al"]` and bucket `1` is `["bob", "beth"]`. All the other buckets are empty.
- Passing that array to `letters_used` gives `"ab"`.
- `letters_used(&bucket_by_initial(&["zoo", "Yak", "zebra"]))` is `"yz"`.
