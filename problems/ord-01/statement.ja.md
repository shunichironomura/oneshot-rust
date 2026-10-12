`1.10.2` や `2.0.0-rc.1` のようなソフトウェアのバージョンは部分ごとに比べるもので、文字列として比べると
間違った答えになります。下の型 `Version` の順序と、最新の安定版を探す関数を実装してください。
`Version` は、解析（`FromStr`）と表示（`Display`）の実装とともにエディタに入っています。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

impl Ord for Version { /* 実装する */ }
impl PartialOrd for Version { /* 実装する */ }

pub fn latest_stable(versions: &[Version]) -> Option<&Version>
```

`pre` は `2.0.0-rc.1` の `rc.1` のようなプレリリースのラベルで、リリース版では `None` です。
与えられた解析は `MAJOR[.MINOR[.PATCH]][-PRE]` の形を読み、`MINOR` や `PATCH` を省略すると 0 になります。

- **順序**
  - まず `major`、次に `minor`、次に `patch` を数として比べます。
  - 3 つとも等しければ、プレリリースのないバージョンのほうが、プレリリースのあるバージョンより
    **大きい**とします（`1.0.0-rc.1` < `1.0.0`）。
  - どちらにもプレリリースがあれば、ラベルを文字列としてバイトごとに比べます（`str` の `<` の順序）。
    つまり `alpha` < `alpha.1` < `beta`、`Beta` < `alpha`、`rc.10` < `rc.2` です。
  - 4 つのフィールドがすべて等しいとき、2 つのバージョンは等しいとします。`"1.2"` と `"1.2.0"` は
    解析すると同じバージョンになります。
  - `Ord` と `PartialOrd` はこれらの規則に従い、互いに、また `==` とも食い違ってはいけません。
    テストでは `==`、`<`、`>`、`cmp`、`partial_cmp` で比べ、`sort()`、`max()`、`min()` を呼び、
    `BTreeSet` と `HashSet` に入れます。
- `latest_stable` は、プレリリースのないバージョンのうち最大のものを返します。そのようなバージョンが
  なければ（`versions` が空のときも）`None` を返します。
- `Version` の定義は変えないでください。

### 例

以下では、`1.9` は `"1.9".parse::<Version>().unwrap()` を表します（ほかも同様です）。

- `1.9` < `1.10` です。また `1.0.0-alpha` < `1.0.0-alpha.1` < `1.0.0-beta` < `1.0.0` < `1.0.1` です。
- `1.2`、`1.2.0`、`1.2.0-rc1` を入れた `BTreeSet` の要素数は 2 です。
- `latest_stable(&[1.0.0, 2.0.0-rc1, 1.4])` は `Some(&1.4.0)` です。
