次のコードはコンパイルが通り、作られる木の形も正しく見えます。
しかし Miri で実行すると、最後に**メモリリーク**が報告されます。

`Node` は家系図の 1 人を表します（エディタに入っています）。家系図は次の関数で作り、読み取ります。

```rust
pub struct Node {
    pub name: String,
    // 非公開のフィールド
}

pub fn node(name: &str) -> Rc<Node>
pub fn adopt(parent: &Rc<Node>, child: &Rc<Node>)
pub fn parent_name(n: &Node) -> Option<String>
pub fn path(n: &Rc<Node>) -> String
pub fn child_names(n: &Node) -> Vec<String>
```

- `node` は、指定した名前を持ち、親も子もいないノードを作ります。別々のノードが同じ名前を持つこともあります。
- `adopt` は `child` を `parent` の最後の子にします。`child` にすでに親がいる場合は（それが `parent` 自身でも）、
  先にその親の子から取り除きます。ノードを自分自身の祖先にするような `adopt` の呼び出しはありません。
- 親は子を所有します。ノードは、呼び出し側がそのノードへの `Rc` を持っている間、**または**生きているノードの子である間は
  生きています。子は親を生かしません。ノードが解放されると、まだ生きているその子たちには親がいなくなります。
- `parent_name` は `n` の親の名前を返します。`n` に親がいなければ `None` です。
- `path` は `n` から親をたどれるところまでたどり、いちばん上のノードから `n` までの名前を `/` でつないで返します。
  親のいないノードなら、その名前だけです。
- `child_names` は `n` の子の名前を、最初の子から最後の子の順に返します。
- `Rc::strong_count(&x)` は、呼び出し側が持っている `x` への `Rc` の数に、`x` に親がいれば 1 を足した値です。
  `Rc::weak_count(&x)` は、呼び出し側が持っている `x` への `Weak` の数に、`x` の子の数を足した値です。

リークを直して、上の規則で生きていないことになったノードがすぐに解放されるようにしてください。
公開されているシグネチャと公開フィールド `name` は変えないでください。`Node` の非公開フィールドは変更して構いません。

### 例

```rust
let root = node("root");
let a = node("a");
let b = node("b");
adopt(&root, &a);
adopt(&a, &b);
// path(&b) == "root/a/b"、parent_name(&a) == Some("root")、child_names(&root) == ["a"]
// Rc::strong_count(&root) == 1、Rc::weak_count(&root) == 1、Rc::strong_count(&a) == 2

drop(root);
// path(&b) == "a/b"、parent_name(&a) == None
```
