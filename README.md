# Rust 学習リポジトリ

「The Rust Programming Language」（第3版・全21章）を体系的に学ぶためのリポジトリ。

## フェーズ構成

| Phase | テーマ | 章 | 内容 |
|-------|--------|-----|------|
| 1 | 基礎 | Ch01-06 | 所有権モデル・基本構文の習得 |
| 2 | プログラムの成長 | Ch07-11 | モジュール・コレクション・エラー処理・テスト |
| 3 | 実践プロジェクト | Ch12-14 | CLI開発・イテレータ・Cargoの深掘り |
| 4 | 高度な概念 | Ch15-19 | スマートポインタ・並行性・非同期・パターン |
| 5 | マスタリー | Ch20-21 | unsafe・マクロ・Webサーバ構築 |

## ディレクトリ構成

```
rust-learning/
├── phase-1-foundations/       # Ch01-06: 基礎
├── phase-2-growing-programs/  # Ch07-11: プログラムの成長
├── phase-3-project-and-functional/ # Ch12-14: 実践プロジェクト
├── phase-4-advanced-concepts/ # Ch15-19: 高度な概念
├── phase-5-mastery/           # Ch20-21: マスタリー
└── extras/playground/         # 実験用スクラッチスペース
```

各章のディレクトリには、Cargoプロジェクトと `notes.md`（学習ノート）が含まれる。

## 学習アプローチ（各章共通）

1. **読む** — 章のテキストを読む
2. **手で打つ** — コードは必ず自分で入力する（コピペしない）
3. **実験する** — 書いたコードを変えて壊して直す
4. **notes.md** — 学んだことをメモ
5. **commit** — `ch04: complete ownership and borrowing exercises` のように説明的に

## ツーリング

```bash
# フォーマット
cargo fmt

# リント（Clippyの指摘自体が良い学習教材）
cargo clippy

# ビルド & 実行
cargo build
cargo run
```

エディタには `rust-analyzer` 拡張を推奨。

## 教材の参照先

- オンライン版: https://doc.rust-lang.org/book/
- ローカル: `./book/`

## 進捗

進捗状況は [PROGRESS.md](./PROGRESS.md) を参照。
