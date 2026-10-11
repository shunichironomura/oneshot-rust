`sort_by` takes a function that returns an `Ordering`. Chain the keys with `then_with`: it looks at
the next key only when the previous ones are `Equal`. To sort a key in **descending** order, swap the
operands (`b.score.cmp(&a.score)`).

`dedup_by` removes **consecutive** duplicates and keeps the first of each run. So sort by name first,
with each player's best run at the front of their group, and the first element kept is exactly the
best run. Then sort once more for the final order. Writing the "better run" comparison once as a
function keeps the two sorts consistent.

### Alternatives

Keep each player's best run in a `HashMap`, then sort with `sort_by_key`. `std::cmp::Reverse` flips
the order of one key inside a tuple. The key closure cannot return a borrow of the element, so the
name is cloned.

```rust
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn leaderboard(runs: Vec<Run>) -> Vec<Run> {
    let mut best: HashMap<String, Run> = HashMap::new();
    for run in runs {
        let better = match best.get(&run.name) {
            Some(b) => (run.score, Reverse(run.time)) > (b.score, Reverse(b.time)),
            None => true,
        };
        if better {
            best.insert(run.name.clone(), run);
        }
    }
    let mut result: Vec<Run> = best.into_values().collect();
    result.sort_by_key(|r| (Reverse(r.score), r.time, r.name.clone()));
    result
}
```
