`1.10.2` や `2.0.0-rc.1` のようなソフトウェアのバージョンは部分ごとに比べるもので、文字列として比べると
間違った答えになります。下の型 `Version` の解析・表示・順序と、最新の安定版を探す関数を実装してください
（`Version` と `ParseVersionError` はエディタに入っています）。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVersionError;

impl FromStr for Version { /* type Err = ParseVersionError; */ }
impl fmt::Display for Version { /* 実装する */ }
impl Ord for Version { /* 実装する */ }
impl PartialOrd for Version { /* 実装する */ }

pub fn latest_stable(versions: &[Version]) -> Option<&Version>
```

- **解析**（`from_str`。`s.parse::<Version>()` はこれを使います）：正しい文字列は
  `MAJOR[.MINOR[.PATCH]][-PRE]` の形をしています。角かっこの部分は省略できます。
  - `MAJOR`、`MINOR`、`PATCH` はそれぞれ 1 文字以上の ASCII 数字（`0` ～ `9`）で、値は `u64` に
    収まらなければなりません。先頭の 0 は許されます（`007` は 7 です）。`MINOR` や `PATCH` を省略すると
    0 になり、`-PRE` がなければフィールド `pre` は `None` になります。
  - `PRE` はプレリリースのラベルで、1 文字以上です。各文字は ASCII の英字、ASCII 数字、`.` の
    いずれかです。書かれたとおりに `pre` に入れます。
  - それ以外のものは許されません。文字列の前後の空白は取り除かず、ほかの文字があったり、空の部分が
    あったりすれば不正です。不正な文字列には `Err(ParseVersionError)` を返します。たとえば `""`、
    `"1..2"`、`"1.2.3.4"`、`"1.2-"`、`"v1"` はどれも不正です。
- **表示**（`Display`）は `MAJOR.MINOR.PATCH` を先頭に 0 を付けない 10 進数で出力し、`pre` が `Some` なら
  続けて `-PRE` を出力します。たとえば `"1.2"` も `"01.2.0"` も `1.2.0` と表示されます。
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
- `Version` と `ParseVersionError` の定義は変えないでください。

### 例

```rust
"1.2".parse::<Version>()      // Ok(Version { major: 1, minor: 2, patch: 0, pre: None })
"3-rc.1".parse::<Version>()   // Ok(Version { major: 3, minor: 0, patch: 0, pre: Some("rc.1") })
"1.2.3.4".parse::<Version>()  // Err(ParseVersionError)
```

以下では、`1.9` は `"1.9".parse::<Version>().unwrap()` を表します（ほかも同様です）。

- `1.2` は `1.2.0` と、`1.0.0-beta` は `1.0.0-beta` と表示されます。
- `1.9` < `1.10` です。また `1.0.0-alpha` < `1.0.0-alpha.1` < `1.0.0-beta` < `1.0.0` < `1.0.1` です。
- `1.2`、`1.2.0`、`1.2.0-rc1` を入れた `BTreeSet` の要素数は 2 です。
- `latest_stable(&[1.0.0, 2.0.0-rc1, 1.4])` は `Some(&1.4.0)` です。
