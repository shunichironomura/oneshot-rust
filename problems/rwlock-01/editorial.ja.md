読み取りガードは、問題文の「ビュー」そのものです。`RwLockReadGuard<'_, Settings>` は設定に参照外しでき、
読み取りガードはいくつでも同時に存在できます（同じスレッドでも、別々のスレッドでも構いません）。
そして読み取りガードがある間は、書き込みは入ってきません。そのため `read` は
`self.settings.read().unwrap()` を返すだけで済みます。シグネチャの `+ '_` が、ガードを `&self` に結び付けています。
`update` は書き込みガードを取ります。書き込みガードは、読み取りガードがすべてなくなるまで待ちます。
書き込みガードは `Settings` への可変な参照外しができるので、`f(&mut settings)` と書けます。

`Mutex` を使ってもコンパイルは通ります。`MutexGuard` も `Deref` を実装しているからです。しかし Mutex の持ち主は
一度に 1 人だけです。同じスレッドで 2 回目の `read()` を呼ぶと 1 つ目のガードが手放されるのを待ちますが、
そのスレッド自身が手放すことはありません。問題文の例の時点でデッドロックし、Miri は
「the evaluated program deadlocked」と報告します。Mutex は再入可能ではありません。また、別々のスレッドの
読み手どうしも互いに待たせてしまいます。

`try_update` は待ってはいけません。`try_write` は `TryLockResult` を返し、`Err(TryLockError::WouldBlock)` は
ビューかほかの更新がロックを持っていることを表すので、これを `None` にします。代わりに `write()` を呼ぶと、
同じスレッドが持っているビューを永遠に待つことになり、`try_write().unwrap()` はパニックします。
先にガードを手に入れ、手に入ってから `f` を呼んでください。そうしないと、更新に失敗したときにも `f` が
実行されてしまいます。`Poisoned` はロックを持ったままパニックした後にしか起きず、`f` はパニックしないので、
ここでは起きません。

バージョンはロックの隣に置いた `AtomicU64` で、`version()` はロックを取らずに読めます。バージョンは書き込みガードが
まだ生きている間に増やすので、バージョンと設定は一緒に変わります。ビューを持っているスレッドからは、そのビューに
対応するバージョンが見えます。`fetch_add` は*前の*値を返すので `+ 1` しています。更新の順序はロックがすでに決めているので、
`Relaxed` で十分です。

ハーネスは `&Store` を複数のスレッドで共有するので、`Store` は `Sync` でなければなりません。`RefCell` や `Cell` を
使うとハーネスがコンパイルできません。`RwLock<Settings>` と `AtomicU64` は `Sync` です。

もう 1 つ注意があります。標準ライブラリの `RwLock` は、同じスレッドが 2 つ目の読み取りロックを取れることを
約束していません。Linux を含むいくつかのプラットフォームでは、書き込みを待っているスレッドがあると、新しい
`read()` も待たされます。そのため、ビューを持っているスレッドが、ほかのスレッドが `update` で待っている間に
もう 1 つビューを取ろうとするとデッドロックします。ハーネスがそうしないのはこのためです。実際のコードでも、
読み取りロックを入れ子にするのは避けましょう。

### 別解

設定とバージョンを 1 つのロックで守る方法です。この場合、読み取りガードは `Settings` ではなく状態全体に
参照外しされます。`&self.state.read().unwrap().settings` を返そうとしても、`read` から戻るときに破棄される
ガードを借用しているのでコンパイルできません（`RwLockReadGuard::map` を使えば書けますが、まだ安定化されていません）。
ガードを持って `Deref` を実装する小さなラッパーを作れば解決します。

```rust
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub theme: String,
    pub font_size: u32,
}

struct State {
    settings: Settings,
    version: u64,
}

pub struct Store {
    state: RwLock<State>,
}

/// A read guard of the whole state that only shows the settings.
struct View<'a>(RwLockReadGuard<'a, State>);

impl Deref for View<'_> {
    type Target = Settings;

    fn deref(&self) -> &Settings {
        &self.0.settings
    }
}

impl Store {
    pub fn new(initial: Settings) -> Store {
        Store { state: RwLock::new(State { settings: initial, version: 1 }) }
    }

    pub fn read(&self) -> impl Deref<Target = Settings> + '_ {
        View(self.state.read().unwrap())
    }

    pub fn update(&self, f: impl FnOnce(&mut Settings)) -> u64 {
        apply(self.state.write().unwrap(), f)
    }

    pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
        match self.state.try_write() {
            Ok(state) => Some(apply(state, f)),
            Err(TryLockError::WouldBlock) => None,
            Err(TryLockError::Poisoned(e)) => panic!("{e}"),
        }
    }

    pub fn version(&self) -> u64 {
        self.state.read().unwrap().version
    }

    pub fn snapshot(&self) -> Settings {
        self.state.read().unwrap().settings.clone()
    }
}

fn apply(mut state: RwLockWriteGuard<'_, State>, f: impl FnOnce(&mut Settings)) -> u64 {
    f(&mut state.settings);
    state.version += 1;
    state.version
}
```

模範解答の `try_update` は、`try_write().ok()?` を使うと短く書けます。ロックが毒されている（poisoned）場合も
`None` になりますが、`f` はパニックしないので、ここでは問題になりません。

```rust
pub fn try_update(&self, f: impl FnOnce(&mut Settings)) -> Option<u64> {
    let guard = self.settings.try_write().ok()?;
    Some(self.apply(guard, f))
}
```
