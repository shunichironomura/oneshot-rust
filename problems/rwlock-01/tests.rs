use std::cell::Cell;
use std::panic::{self, AssertUnwindSafe};
use std::sync::Barrier;
use std::thread;

use crate::judge::{case, check, check_eq};
use crate::user::{Settings, Store};

fn settings(theme: &str, font_size: u32) -> Settings {
    Settings { theme: theme.to_string(), font_size }
}

/// Calls `try_update` and returns its result and whether it called `f`. A panic is caught, so that
/// another thread waiting at a barrier is never left behind.
fn try_and_see(store: &Store) -> Result<(Option<u64>, bool), String> {
    let called = Cell::new(false);
    let result = panic::catch_unwind(AssertUnwindSafe(|| store.try_update(|_| called.set(true))));
    result.map(|r| (r, called.get())).map_err(|_| "try_update panicked".to_string())
}

pub fn run() {
    case("sample-1", || {
        let store = Store::new(settings("dark", 12));
        let a = store.read();
        let b = store.read();
        check_eq(&a.theme, &b.theme)?;
        check_eq(b.font_size, 12)?;
        check_eq(store.try_update(|s| s.font_size = 14), None)?;
        drop(a);
        drop(b);
        check_eq(store.try_update(|s| s.font_size = 14), Some(2))
    });
    case("sample-2", || {
        let store = Store::new(settings("dark", 12));
        check_eq(store.try_update(|s| s.font_size = 14), Some(2))?;
        check_eq(store.update(|s| s.theme = "light".to_string()), 3)?;
        check_eq(store.version(), 3)?;
        check_eq(store.snapshot(), settings("light", 14))
    });
    case("hidden-1", || {
        // Views show the settings as they are when they are taken; snapshots do not change.
        let store = Store::new(settings("solarized", 10));
        check_eq(store.version(), 1)?;
        check_eq(&*store.read(), &settings("solarized", 10))?;
        let before = store.snapshot();
        check_eq(store.update(|s| s.theme.push_str("-dark")), 2)?;
        let view = store.read();
        check_eq(&view.theme, "solarized-dark")?;
        check_eq(view.font_size, 10)?;
        drop(view);
        check_eq(before, settings("solarized", 10))?;
        check_eq(store.try_update(|s| s.font_size *= 3), Some(3))?;
        check_eq(store.update(|_| {}), 4)?;
        check_eq(store.version(), 4)?;
        check_eq(store.snapshot(), settings("solarized-dark", 30))
    });
    case("hidden-2", || {
        // A failed try_update calls nothing and changes nothing, not even the version.
        let store = Store::new(settings("dark", 12));
        let called = Cell::new(false);
        let view = store.read();
        check_eq(
            store.try_update(|s| {
                called.set(true);
                s.font_size = 99;
            }),
            None,
        )?;
        check(!called.get(), "try_update called f while a view was alive")?;
        check_eq(store.version(), 1)?;
        check_eq(store.snapshot(), settings("dark", 12))?;
        check_eq(view.font_size, 12)?;
        drop(view);
        check_eq(store.update(|s| s.font_size = 13), 2)?;
        check_eq(store.version(), 2)?;
        let view = store.read();
        check_eq(store.try_update(|_| called.set(true)), None)?;
        check(!called.get(), "try_update called f while a view was alive")?;
        drop(view);
        check_eq(store.try_update(|s| s.theme = "high contrast".to_string()), Some(3))?;
        check_eq(store.snapshot(), settings("high contrast", 13))
    });
    case("hidden-3", || {
        // try_update fails as long as at least one of several views is alive.
        let store = Store::new(settings("dark", 12));
        let mut views: Vec<_> = (0..3).map(|_| store.read()).collect();
        check(views.iter().all(|v| v.font_size == 12), "every view shows the settings")?;
        check_eq(store.try_update(|s| s.font_size = 1), None)?;
        views.truncate(1);
        check_eq(store.try_update(|s| s.font_size = 1), None)?;
        check_eq(store.version(), 1)?;
        drop(views);
        check_eq(store.try_update(|s| s.font_size = 1), Some(2))?;
        check_eq(store.read().font_size, 1)
    });
    case("hidden-4", || {
        // Another thread holds a view, then runs an update. try_update fails both times.
        let store = Store::new(settings("dark", 12));
        let barrier = Barrier::new(2);
        let (mine, during_view, during_update, theirs) = thread::scope(|scope| {
            let other = scope.spawn(|| {
                let view = store.read();
                barrier.wait(); // the view is alive now
                barrier.wait(); // main is done
                let seen = view.font_size;
                drop(view);
                let version = store.update(|s| {
                    barrier.wait(); // the update is running now
                    barrier.wait(); // main is done
                    s.font_size = 30;
                });
                (seen, version)
            });
            barrier.wait();
            // Views on two threads are alive at the same time.
            let mine = store.read().font_size;
            let during_view = try_and_see(&store);
            barrier.wait();
            barrier.wait();
            let during_update = try_and_see(&store);
            barrier.wait();
            (mine, during_view, during_update, other.join().unwrap())
        });
        check_eq((mine, during_view?), (12, (None, false)))?;
        check_eq(during_update?, (None, false))?;
        check_eq(theirs, (12, 2))?;
        check_eq(store.version(), 2)?;
        check_eq(store.snapshot(), settings("dark", 30))
    });
    case("hidden-5", || {
        // Three threads (main included) read and update at the same time; no update is lost.
        let store = Store::new(settings("dark", 12));
        let work = || -> Result<Vec<u64>, String> {
            let mut versions = Vec::new();
            for _ in 0..8 {
                {
                    let view = store.read();
                    let size = view.font_size;
                    thread::yield_now();
                    if view.font_size != size {
                        return Err("a view changed while it was alive".to_string());
                    }
                }
                let version = store.update(|s| s.font_size += 1);
                if store.version() < version {
                    return Err("version() went back".to_string());
                }
                versions.push(version);
            }
            Ok(versions)
        };
        let results = thread::scope(|scope| {
            let others = [scope.spawn(work), scope.spawn(work)];
            let mine = work();
            let mut results: Vec<_> = others.into_iter().map(|h| h.join().unwrap()).collect();
            results.push(mine);
            results
        });
        let mut all = Vec::new();
        for versions in results {
            let versions = versions?;
            check(versions.windows(2).all(|w| w[0] < w[1]), "a thread got decreasing versions")?;
            all.extend(versions);
        }
        all.sort();
        check_eq(all, (2..=25).collect::<Vec<u64>>())?;
        check_eq(store.version(), 25)?;
        check_eq(store.snapshot(), settings("dark", 36))
    });
    case("hidden-6", || {
        // Only updates, on three threads at the same time: each version is returned exactly once.
        let store = Store::new(settings("dark", 0));
        let work = || (0..10).map(|_| store.update(|s| s.font_size += 1)).collect::<Vec<u64>>();
        let mut all = thread::scope(|scope| {
            let others = [scope.spawn(work), scope.spawn(work)];
            let mut all = work();
            for other in others {
                all.extend(other.join().unwrap());
            }
            all
        });
        all.sort();
        check_eq(all, (2..=31).collect::<Vec<u64>>())?;
        check_eq(store.version(), 31)?;
        check_eq(store.snapshot(), settings("dark", 30))
    });
}
