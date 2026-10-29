A closure that changes a captured variable borrows it mutably, and that borrow lasts as long as the
closure may still be called: until its **last use**, not just during each call. In the loop,
`record` is called again in the next iteration, so it holds `&mut errors` and `&mut log` across the
whole loop, and reading `errors` between two calls is rejected (E0503, "cannot use `errors` because
it was mutably borrowed").

The fix is to let the closure hand out what the loop needs: here it returns the error count after
recording the event. `log.push("abort")` in the loop can stay. The borrow checker follows the
control flow: right after that push comes `break`, so on that path `record` is never called again
and its borrow of `log` has already ended. For the same reason, `(errors, log)` after the loop is
fine.

Adding `move` to `record` does not help. `errors` is `Copy`, so the closure would count in its own
copy while the loop keeps seeing `0`, and `log` would be moved into the closure, out of the loop's
reach.

### Alternatives

Write the closure's body directly in the loop. Without a closure, nothing holds a borrow between
the steps.

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>) {
    let mut errors = 0;
    let mut log = Vec::new();
    for event in events {
        if event.starts_with("ERR") {
            errors += 1;
        }
        log.push(event.to_string());
        if errors == limit {
            log.push("abort".to_string());
            break;
        }
    }
    (errors, log)
}
```

Or pass the state as arguments. A closure that captures nothing holds no borrow, and each call
borrows `errors` and `log` only while it runs.

```rust
pub fn tally(events: &[&str], limit: usize) -> (usize, Vec<String>) {
    let mut errors = 0;
    let mut log = Vec::new();
    let record = |errors: &mut usize, log: &mut Vec<String>, event: &str| {
        if event.starts_with("ERR") {
            *errors += 1;
        }
        log.push(event.to_string());
    };
    for event in events {
        record(&mut errors, &mut log, event);
        if errors == limit {
            log.push("abort".to_string());
            break;
        }
    }
    (errors, log)
}
```
