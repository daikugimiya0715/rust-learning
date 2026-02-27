# Rust 学習メモ（目次）

| ファイル | 内容 |
|---|---|
| [macro.md](macro.md) | マクロ |
| [ownership.md](ownership.md) | 所有権・ムーブ・Copy トレイト・スタックとヒープ |
| [string-types.md](string-types.md) | 文字列型の違い（`str`, `&str`, `String`, `&String`） |
| [struct.md](struct.md) | 構造体・`Debug`/`Display` トレイト・`derive` マクロ |
| [compiler.md](compiler.md) | コンパイラの警告（dead_code 警告など） |
| [error-handling.md](error-handling.md) | エラー処理（`panic!`・`Result`） |
| [generics-traits-lifetimes.md](generics-traits-lifetimes.md) | ジェネリック型・トレイト・ライフタイム |

## 2026-02-19

- `#[derive(...)]` は手続き的マクロ（derive マクロ）の一種。コンパイラがトレイトの `impl` を自動生成する仕組み
- `use` はどのモジュール内でも書ける。`mod tests` 内の `use super::*;` はテスト専用構文ではなく、子モジュールから親の要素を持ち込む通常の `use`
- `lib.rs` でテストからしか使わない非 `pub` 要素は `dead_code` 警告が出る。`#[cfg(test)]` は通常ビルド時にコンパイルされないため「誰も使っていない」扱いになる
- 11章テスト: `sandbox/adder/` に学習資料を整理済み（ユニットテスト + 結合テスト + common モジュール）

## 2026-02-24

- 12章 minigrep（12.1〜12.6）: `sandbox/minigrep/` に全章を整理
  - 12.3: main.rs は薄く、ロジックは lib.rs へ → テスト可能に
  - 12.4: TDD で `search` 関数を開発。戻り値のライフタイムは `contents` に紐づく（`query` ではない）
  - 12.5: `env::var("IGNORE_CASE").is_ok()` で環境変数の有無を bool に。`to_lowercase()` は新しい String を返す
  - 12.6: エラーは `eprintln!`（stderr）に出力。`> file` でリダイレクトしてもエラーは画面に残る
  - `Box<dyn Error>` は今は「どんなエラーでも返せる型」と理解すればOK（Box→15章、dyn→17章で詳しく）

- 13章 関数型言語の機能（13.1〜13.4）
  - 13.1 クロージャ:
    - クロージャは `||` で定義する無名関数。環境の変数をキャプチャできる
    - キャプチャ方法は3つ: 不変借用（`Fn`）、可変借用（`FnMut`）、所有権移動（`FnOnce`）
    - `move` キーワードで強制的に所有権を移動（`thread::spawn` で必要）
    - `Fn ⊂ FnMut ⊂ FnOnce` の包含関係。`sort_by_key` は複数回呼ぶので `FnMut` を要求
    - **なぜ Fn トレイトが必要か**: クロージャは定義するたびに毎回別の匿名型になる。標準ライブラリが「クロージャを受け取る関数」を書くには、トレイトで抽象化するしかない
    - **2つの視点**: (1) 作る側 — コンパイラが本体を見て `Fn`/`FnMut`/`FnOnce` を自動実装する (2) 受け取る側 — `sort_by_key` 等がトレイト境界で要求を指定する。両方が噛み合って型安全を実現
  - 13.2 イテレータ:
    - `Iterator` トレイトの必須メソッドは `next()` のみ（`Option<Self::Item>` を返す）
    - `iter()`=不変参照、`iter_mut()`=可変参照、`into_iter()`=所有権移動
    - 消費アダプタ（`sum`, `collect`）はイテレータを消費、イテレータアダプタ（`map`, `filter`）は怠惰
  - 13.3 minigrep 改善:
    - `Config::build` を `&[String]` → `impl Iterator<Item = String>` に変更し `clone()` を除去
    - `search` を `lines().filter().collect()` のイテレータチェーンに書き換え
  - 13.4: イテレータはゼロコスト抽象化。手書きループと同等の機械語にコンパイルされる
