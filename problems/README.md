# Writing problems

English | [日本語](README.ja.md)

Each problem lives in its own directory. The directory name is the problem ID.

Problems are published under [CC BY 4.0](LICENSE). If a problem is based on examples or code from other material,
add it to the sources tables in both the root [README.md](../README.md#sources) and
[README.ja.md](../README.ja.md#出典) (only if the original license is compatible with CC BY 4.0).

```text
problems/<id>/
  problem.toml      Metadata
  statement.md      Problem statement (English, required)
  statement.ja.md   Problem statement (Japanese, optional; the English one is shown if missing)
  template.rs       Starting code
  tests.rs          Test harness
  solution.rs       Reference solution (shown to players who finished the problem; served as a per-problem chunk)
  editorial.md      Editorial (English, optional; shown with the reference solution)
  editorial.ja.md   Editorial (Japanese, optional; only if editorial.md exists)
```

## problem.toml

```toml
id = "lifetime-01"             # same as the directory name
title = { en = "Longest word", ja = "最長の単語" }   # en is required
difficulty = 2                 # difficulty tier: 1 (beginner) | 2 (intermediate) | 3 (advanced); used for selection and list order
concepts = ["lifetimes", "iterators"]  # notes for authors; not shown to players since they are spoilers
kind = "implement"             # implement | repair | signature

[judge]                        # omitted fields take the defaults below
forbid_unsafe = true           # add #[forbid(unsafe_code)] to the player's code
miri_checks = "full"           # full | fast (fast disables Stacked Borrows and validation)
time_limit_ms = 15000
stdout_limit_bytes = 65536
harness_visible = false        # whether to show tests.rs to the player
```

A round consists of one beginner, one intermediate, and one advanced problem, in that order (`ROUND_LADDER` in
`web/src/core/round.ts`). Within each tier, problems not yet played are preferred, so make sure every tier has enough
problems.

## Problem kinds

| kind | Description | template.rs |
| --- | --- | --- |
| `implement` | Given a spec and signatures, implement the bodies | Must compile and must not get AC |
| `repair` | Fix code that doesn't compile | Must be CE |
| `signature` | Given the bodies, write signatures that make the harness pass | Must be CE. `harness_visible = true` is required |

## tests.rs

The player's code is `crate::user`, and the judge library is `crate::judge`.

```rust
use crate::judge::{case, check, check_eq};
use crate::user::longest_word;

pub fn run() {
    case("example-1", || check_eq(longest_word("a bb ccc"), "ccc"));
    case("case-1", || check(longest_word("").is_empty(), "should be empty"));
}
```

- `case(id, f)`: one test case. `f` returns `Result<(), String>`. A panic counts as FAIL, and the remaining cases
  still run.
- `check_eq(got, expected)` / `check(cond, message)`: helpers for checking.
- Name the cases shown in the statement's examples `example-N` and the others `case-N`. Cases starting with
  `example-` are public; the others are hidden, and their failure reasons are not shown.
  However, in problems with `harness_visible = true`, the harness code is visible, so all cases are treated as public.
- Case IDs are obtained by extracting `case("...")` from `tests.rs`. Write each ID directly as a string literal.

## Size guidelines

A round's score is the number of submissions, so the purpose of these guidelines is to avoid adding chances for
compile errors or WA in places unrelated to the theme. Follow these principles before the numbers.

1. **One theme per problem.** The first entry of `concepts` is the main theme. Every hole the player fills in
   (`todo!()`, a fix, a signature) must use the main theme. Holes that don't use it should either be given already
   implemented in the template or removed.
2. **One task per problem.** All holes must belong to a single task. If, even within the same theme, there are two or
   more tasks that can be specified and tested independently, make them separate problems. Otherwise, a player who
   has solved one still can't submit until they finish the other, and every WA on one costs submissions for the
   other too.
   Example: a `Triage` type and a function `k_smallest` that doesn't use it become separate problems. A set of
   methods on one type, or a function that uses a type or function the player wrote, counts as one task.
3. **Supporting concepts must be "free".** Concepts other than the main theme must be ones that can be assumed known
   at that tier and that don't cause extra compile errors or WA when solved the natural way.
   Example: at the beginner tier, `match` and `Vec::push` are free. `collect` into a `Result` and special handling
   of empty input are not.
4. **Spec details serve the theme.** If the reference solution has a branch that is unrelated to the main theme and
   exists only to pass a hidden test, remove that hidden test and the spec.
5. **When over the limit, cut first. If that's not enough, split.** Cut off-theme spec, duplicate variants or methods
   of the same shape, and add-on specs that just count and return results. Splitting is needed only when there are
   two themes or two tasks. The second problem may provide what was implemented in the first in its template.
6. **The statement fits on one screen.** A reader should know what to write within a minute. Ideally, each bullet
   point corresponds one-to-one with a hidden test. Don't test behavior that isn't in the statement.

The numeric limits are as follows (`SIZE_LIMITS` in `web/content/size.ts`). Exceeding one is a signal to consider
cutting or splitting, not a prohibition. A problem that satisfies principles 1–4 may stay even if it exceeds a limit.

| Metric | How it is measured | Beginner | Intermediate | Advanced |
| --- | --- | --- | --- | --- |
| Lines to write (implement) | lines in `solution.rs` − lines in `template.rs` + number of `todo!()` | 20 | 30 | 40 |
| Fixes (repair / signature) | Number of logically independent fixes, counted by hand (the diff hunk count from the script is a reference) | 3 | 4 | 5 |
| Statement words | Word count of `statement.md` (English) | 400 | 500 | 600 |
| Test cases | Number of `case("` in `tests.rs` | 10 | 10 | 10 |

Run the following in `web/` to list the measurements of all problems and those over a limit. CI runs it too, but
only as a warning, never a failure.

```sh
node scripts/problem-size.ts            # all problems
node scripts/problem-size.ts iter-01    # only the given problems
```

## Reference solutions and editorials

`solution.rs` is shown to players too. Write it as idiomatic, readable code, assuming players will read it.
In `editorial.md`, briefly cover the key points (why it is written that way, common pitfalls) and alternative
solutions. Alternative solution code must also compile and pass the tests.

The display conditions are as follows (`web/src/core/solution.ts`).

- On the play screen, the "Solution" tab appears once the attempt is finished (AC or give up). It can't be viewed
  mid-attempt.
- On a problem's history page (opened from the records page), it can be viewed if an attempt at that problem has
  ever been finished.
- Once viewed, later attempts (and suspended ones) are marked "Solution seen" and don't update the personal best.

Reference solutions are not included in the main bundle; each problem's chunk is loaded when it is shown.
Judging happens in the browser, so, like the hidden tests, they are not technically secret.

## Verification

Run the following in `web/` to confirm that the reference solution gets AC, that the template meets the conditions
in the table above, and that the reference solution finishes within half the time limit.

```sh
node scripts/verify-problems.ts            # all problems
node scripts/verify-problems.ts iter-01    # only the given problems
```
