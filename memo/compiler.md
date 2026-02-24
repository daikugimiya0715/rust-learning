# コンパイラの警告（dead_code 警告など）

## dead_code 警告

- 定義したのに使っていない関数・変数があると警告が出る
- Rust コンパイラは「使われていないコード」を積極的に教えてくれる

```
warning: function `foo` is never used
 --> src/main.rs:1:4
  |
1 | fn foo() {}
  |    ^^^
  = note: `#[warn(dead_code)]` on by default
```

### 警告を抑制する方法

```rust
#[allow(dead_code)]       // この関数だけ警告を抑制
fn unused_function() {}

#![allow(dead_code)]      // ファイル全体で抑制（先頭に書く）
```

- `_` プレフィックスをつけた変数名も警告が出ない: `let _x = 5;`

## unused_variables 警告

```rust
let x = 5;  // warning: unused variable: `x`
let _x = 5; // OK（_ をつけると警告が消える）
```

## よく見る警告レベル

| レベル | 意味 |
|---|---|
| `warning` | コンパイルは通るが、問題の可能性あり |
| `error` | コンパイルが通らない。必ず修正が必要 |

## `#[derive(...)]` 関連の警告

- `Debug` を derive していない型を `{:?}` で表示しようとすると**エラー**になる
- 必要なトレイトは `#[derive(Debug)]` で付与する

## clippy（リンター）

- `cargo clippy` でより詳細なコード品質チェックができる
- コンパイラの警告よりさらに多くのパターンを検出してくれる
