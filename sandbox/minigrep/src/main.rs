// ====================================================================
// 12章: minigrep — コマンドラインプログラム (main.rs)
// ====================================================================
//
// 12.1: コマンドライン引数を受け付ける
//   - std::env::args() → collect() で Vec<String> に
//
// 12.2: ファイルを読み込む
//   - std::fs::read_to_string()
//
// 12.3: リファクタリング — モジュール性とエラー処理の改善
//   - main は薄く: 設定 → 実行 → エラー処理のみ
//
// 12.4: TDD で search 関数を開発（lib.rs 側）
//
// 12.5: 環境変数で大文字小文字の区別を制御
//   - IGNORE_CASE=1 cargo run -- <query> <file>
//
// 12.6: エラーメッセージを stderr に出力
//   - println! → eprintln! に変更
//   - cargo run -- to poem.txt > output.txt で結果だけファイルに保存できる
//
// 13.3: イテレータを使った改善
//   - Vec<String> に collect する中間ステップを除去
//   - env::args() のイテレータを直接 Config::build に渡す
//
// 使い方:
//   cargo run -- <検索文字列> <ファイルパス>
//   cargo run -- the poem.txt
//   IGNORE_CASE=1 cargo run -- the poem.txt     （大文字小文字を無視）
//   cargo run -- the poem.txt > output.txt       （結果をファイルに保存）

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // 13.3: env::args() のイテレータを直接渡す
    // 以前: let args: Vec<String> = env::args().collect();
    //       Config::build(&args)
    // 改善: 中間の Vec を作らず、イテレータをそのまま渡す
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        // 12.6: eprintln! でエラーを stderr に出力
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // 12.6: こちらも eprintln!
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// ====================================================================
// 12.6: stdout vs stderr
// ====================================================================
//
// println!  → stdout（標準出力）— プログラムの「結果」
// eprintln! → stderr（標準エラー出力）— エラーメッセージ
//
// なぜ分けるのか？
//   $ cargo run -- to poem.txt > output.txt
//   → stdout はファイルにリダイレクトされる
//   → stderr は画面に表示される
//   → エラーが起きたら画面で確認でき、結果だけファイルに保存できる
