`&[u8]` は `std::io::Read`（読み取るとスライスの先頭からバイトが取り除かれます）だけでなく
`BufRead` も実装しているので、Book の `TcpStream` と同じように、バイトスライスを 1 行ずつ読めます。
`lines` は `BufRead` トレイトのメソッドなので、`use std::io::BufRead;` がないと呼び出しが
コンパイルできません。Book のように `BufReader::new(raw)` と書いても動きますが、バッファが 1 つ
増えるだけです。

`lines()` の各要素は `String` ではなく `io::Result<String>` です。行が正しい UTF-8 でないと、
その行の読み取りが失敗します。Book のように `unwrap` すると、そのようなリクエストでパニックします。
`.ok()?` を使えば、エラーを規則どおり `None` にできます。`flatten()`（エラーを読み飛ばす）や
`map_while(Result::ok)`（最初のエラーで止まる）も同じくらい簡単に書けてコンパイルも通りますが、
不正な入力に対してリクエストを返してしまうことがあります。また、`lines()` は行末も取り除く
（`\n` で分け、その直前の `\r` も除く）ので、バージョンをそのまま `"HTTP/1.1"` と比べられます。

ループは最初の空行で止まるので、ボディは読まれず、デコードもされません。先に `raw` 全体を
`str::from_utf8` で変換すると、ボディがバイナリのリクエストを拒否してしまいます。

`split(' ')` は空の部分も残すので、空白が 2 つ続くと空の部分が 1 つ増えます。一方 `split_whitespace` は
`"GET  / HTTP/1.1"` を受け付けてしまいます。スライスパターン
`let [method, path, version] = parts[..] else { ... }` を使うと、個数のチェックと名前付けが一度にできます。
`split_once(": ")` は最初の区切りで分け、区切りがなければ `None` を返すので、`?` でそのまま返せます。

`respond` では、`str::len` が**バイト数**を返し、これが `Content-Length` に必要な値です。
`"こんにちは"` は 5 文字（`chars().count()`）ですが 15 バイトです。`Vec<u8>` は `io::Write` を
実装しているので、`std::io::Write` をスコープに入れれば `write!` で直接書き込めます。`write!` は
`io::Result` を返し、使わないとコンパイラが警告します。`Vec` への書き込みは失敗しないので、
`expect` で済ませています。

### 別解

Book のように、空行までの行を先に集める書き方です。`take_while` は最初の `Ok("")` で止まりますが、
エラーは通すので、`Result<Vec<_>, _>` に `collect` すると最初のエラーが `Err` になります。
ヘッダも `Option<_>` に `collect` すれば同じように扱えます。

```rust
use std::io::{BufRead, BufReader};

pub fn parse_request(raw: &[u8]) -> Option<Request> {
    let lines: Vec<String> = BufReader::new(raw)
        .lines()
        .take_while(|line| !matches!(line, Ok(l) if l.is_empty()))
        .collect::<Result<_, _>>()
        .ok()?;
    let (request_line, header_lines) = lines.split_first()?;

    let parts: Vec<&str> = request_line.split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }

    let headers = header_lines
        .iter()
        .map(|line| {
            let (name, value) = line.split_once(": ")?;
            Some((name.to_string(), value.to_string()))
        })
        .collect::<Option<_>>()?;

    Some(Request { method: method.to_string(), path: path.to_string(), headers })
}
```

`read_line` は、渡した `String` に 1 行を読み込みます。行末の改行は残り、入力の終わりでは `Ok(0)` を
返します。リーダーは `raw` そのものなので、`raw` を `mut` にする必要があります（読むとスライスが進みます）。

```rust
use std::io::BufRead;

pub fn parse_request(mut raw: &[u8]) -> Option<Request> {
    let mut line = String::new();
    raw.read_line(&mut line).ok()?;
    let parts: Vec<&str> = line.strip_suffix("\r\n").unwrap_or(&line).split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }
    let (method, path) = (method.to_string(), path.to_string());

    let mut headers = Vec::new();
    loop {
        line.clear();
        if raw.read_line(&mut line).ok()? == 0 {
            break;
        }
        let text = line.strip_suffix("\r\n").unwrap_or(&line);
        if text.is_empty() {
            break;
        }
        let (name, value) = text.split_once(": ")?;
        headers.push((name.to_string(), value.to_string()));
    }

    Some(Request { method, path, headers })
}
```

`std::io` を使わずにパースすることもできます。バイト列を `b'\n'` で分け、行ごとにデコードします。
各要素は `Option<&str>`（不正な UTF-8 なら `None`）になるので、最初の行には `??` を使います。
イテレータは遅延評価なので、空行より後（ボディを含む）はデコードされません。

```rust
pub fn parse_request(raw: &[u8]) -> Option<Request> {
    let mut lines = raw.split(|&b| b == b'\n').map(|line| {
        let line = line.strip_suffix(b"\r").unwrap_or(line);
        std::str::from_utf8(line).ok()
    });

    let request_line = lines.next()??;
    let parts: Vec<&str> = request_line.split(' ').collect();
    let [method, path, version] = parts[..] else {
        return None;
    };
    if method.is_empty() || path.is_empty() || version != "HTTP/1.1" {
        return None;
    }

    let mut headers = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let (name, value) = line.split_once(": ")?;
        headers.push((name.to_string(), value.to_string()));
    }

    Some(Request { method: method.to_string(), path: path.to_string(), headers })
}
```

`format!` で `String` を作り、`into_bytes` でコピーせずに `Vec<u8>` に変換する方法です。
`Write` の `use` は要りません。

```rust
pub fn respond(req: &Request) -> Vec<u8> {
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => ("200 OK", "<h1>Hello!</h1>"),
        ("GET", "/greet") => ("200 OK", "こんにちは"),
        _ => ("404 NOT FOUND", "Oops!"),
    };
    format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\n\r\n{body}", body.len()).into_bytes()
}
```
