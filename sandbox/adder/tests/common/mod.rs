// tests/common/mod.rs
// 結合テスト間で共有するヘルパー関数を置く場所
//
// なぜ tests/common.rs ではなく tests/common/mod.rs なのか？
// → tests/ 直下の .rs ファイルはそれぞれ独立したテストクレートとして
//   コンパイルされてしまう。common/mod.rs にすればテストとして扱われない。

pub fn setup() {
    // テスト用の共通セットアップ処理
    // （実際のプロジェクトではDB接続やテストデータ作成などを行う）
    println!("[setup] テスト環境を準備しました");
}
