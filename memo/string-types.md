# 文字列型の違い（`str`, `&str`, `String`, `&String`）

## 一覧

| 型 | 意味 | メモリ | 可変？ | よく使う場面 |
|---|---|---|---|---|
| `str` | 文字列スライス型（サイズ不定） | どこか（静的領域・ヒープなど） | 不可 | 直接使わない（常に参照経由） |
| `&str` | `str` への参照 | ポインタ + 長さ（スタック上） | 不可 | 関数の引数、文字列リテラル |
| `String` | ヒープ上の可変文字列 | ヒープ（所有権あり） | 可 | 文字列の組み立て・変更 |
| `&String` | `String` への参照 | ポインタ（スタック上） | 不可 | ほぼ使わない（`&str` で代用） |

## ポイント

### `&str` と `String` の使い分け（実質これだけ覚えればOK）

- **`&str`**: 読むだけ。文字列リテラル `"hello"` の型はこれ
- **`String`**: 所有して変更したいとき。`String::from("hello")` や `"hello".to_string()` で作る

```rust
let s1: &str = "hello";              // 文字列リテラル → &str
let s2: String = String::from("hello"); // ヒープ上に確保 → String
let s3: &str = &s2;                  // String → &str に変換（Deref）
```

### 関数の引数には `&str` を使う

```rust
// 良い: &str なら String も &str も受け取れる
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

greet("world");                    // &str をそのまま渡せる
greet(&String::from("world"));     // String も &str に自動変換される
```

- `&String` を引数にすると `String` しか受け取れないので、`&str` の方が柔軟

### `str` を直接使わない理由

- `str` はコンパイル時にサイズが決まらない（DST: Dynamically Sized Type）
- 変数に直接置けないので、常に `&str`（参照）として使う

### 変換まとめ

```
String  → &str   : &s または s.as_str()
&str    → String : s.to_string() または String::from(s)
```
