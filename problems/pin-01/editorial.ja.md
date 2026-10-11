`Future::poll` は `self: Pin<&mut Self>` を受け取ります。`Pin::new` でこれを作れるのは、中身が `Unpin`、
つまり動かされても困らない型で、ピン留めしても何も約束しなくてよい場合だけです。`async` ブロックの
Future は `Unpin` ではありません。`.await` をまたいで自分自身の中を指す参照（自分のローカル変数への
参照など）を持つことがある状態機械なので、動かすとその参照がダングリングになるからです。`dyn Future`
も、中身について何もわからないので `Unpin` ではありません。そのため `Pin::new(task.as_mut())` は
「`dyn Future<Output = u32>` cannot be unpinned」（E0277）で拒否されます。コンパイラの提案どおり
`.as_mut()` を外しても、エラーの場所が変わるだけです。`Box<F>` が Future になるのは `F: Unpin` のとき
だけだからです。

直し方は、各 Future を作るときにピン留めすることです。`Box::new` の代わりに `Box::pin` を使います。
`Box::pin` は Future をヒープに移して、その Future をもう二度と動かさないことを約束する `Pin<Box<_>>` を
返します。`Box<_>` が `Box<dyn Future<Output = u32>>` に型強制できるのと同じように、これは
`Pin<Box<dyn Future<Output = u32>>>` に型強制できます。`Pin<Box<_>>` 自体はただのポインタで
`Unpin` なので、`Vec` に集めたり、添字と組にしたり、`retain_mut` で捨てたりと、自由に動かせます。
ポーリングするときは、`task.as_mut()`（`Pin::as_mut`）で `&mut Pin<Box<_>>` を `poll` が受け取る
`Pin<&mut dyn Future<Output = u32>>` として借用し直します。Box を手放す必要はありません。

この直し方では、Box と Pin にかかわる箇所をすべて直すことになります。`Task` 型の別名、2 か所の `Box::new`
（`as Task` でキャストしている方も含む）、そして `poll` の呼び出しです。別名だけを変えると、
`Box::new` のところで型が合わなくなり、`Pin::new` も相変わらず失敗します。反対の例が
`YieldNow` です。`bool` を持つだけの普通の構造体なので `Unpin` であり、自分の `poll` の中では
`Pin<&mut Self>` 越しに `self.yielded` に代入できます。

ほかにも思いつきやすい直し方はありますが、うまくいきません。`Box<dyn Future<Output = u32> + Unpin>`
にすると、今度は `async` ブロックが `Unpin` でないので入らなくなります。`std::pin::pin!` は値を
今のブロックのローカル変数にピン留めするので、実行時に決まる個数の Future を、それを作るループの後まで
生かしておくことはできません。

### 別解

エグゼキュータをジェネリックにして、その中でピン留めする方法です。各関数は自分の `async` ブロックの型の `Vec` を
渡すので、トレイトオブジェクトは要りません。それでも `Box::pin` は必要です。`Vec<F>` の要素は、
ベクタが確保し直したり要素を取り除いたりすると動くので、`unsafe` なしではその場でピン留めできません。

```rust
fn run_round_robin<F: Future<Output = u32>>(tasks: Vec<F>) -> Vec<(usize, u32)> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut unfinished: Vec<(usize, Pin<Box<F>>)> =
        tasks.into_iter().map(Box::pin).enumerate().collect();
    let mut finished = Vec::new();
    while !unfinished.is_empty() {
        unfinished.retain_mut(|(index, task)| match task.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                finished.push((*index, output));
                false
            }
            Poll::Pending => true,
        });
    }
    finished
}

pub fn squares_round_robin(n: u32) -> Vec<u32> {
    let tasks: Vec<_> = (0..n)
        .map(|i| async move {
            let mut square = 0;
            for _ in 0..i {
                square += i;
                yield_now().await;
            }
            square
        })
        .collect();
    let mut squares = vec![0; n as usize];
    for (index, output) in run_round_robin(tasks) {
        squares[index] = output;
    }
    squares
}

pub fn finish_order(delays: &[u32]) -> Vec<usize> {
    let tasks: Vec<_> = delays
        .iter()
        .map(|&delay| async move {
            for _ in 0..delay {
                yield_now().await;
            }
            delay
        })
        .collect();
    run_round_robin(tasks).into_iter().map(|(index, _)| index).collect()
}
```

Box に入れた Future は、Box が所有しているので、後からピン留めすることもできます。`Box::into_pin`
（または `Pin::from`）は `Box<T>` を、`T` を動かさずに `Pin<Box<T>>` に変えます。ピン留めされていない
Box は消費されるので、これは安全です。この場合、`Task` 型の別名と `Box::new` の呼び出しはそのままで、
エグゼキュータだけを変えます。

```rust
fn run_round_robin(tasks: Vec<Task>) -> Vec<(usize, u32)> {
    let mut cx = Context::from_waker(Waker::noop());
    let mut unfinished: Vec<(usize, Pin<Task>)> =
        tasks.into_iter().map(Box::into_pin).enumerate().collect();
    let mut finished = Vec::new();
    while !unfinished.is_empty() {
        unfinished.retain_mut(|(index, task)| match task.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                finished.push((*index, output));
                false
            }
            Poll::Pending => true,
        });
    }
    finished
}
```

タスクを `Pin<Box<_>>` にした後なら、`Pin::new(task).poll(&mut cx)` でもコンパイルが通ります。
`Pin<Box<dyn Future>>` は `Unpin` で、それ自体も Future だからです（`P` が Future への可変なポインタなら、
`Pin<P>` は `Future` を実装します）。
