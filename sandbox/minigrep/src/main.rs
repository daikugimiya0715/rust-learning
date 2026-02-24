// ====================================================================
// 12章: minigrep — コマンドラインプログラム
// ====================================================================
//
// 12.1: コマンドライン引数を受け付ける
//   - std::env::args() でイテレータを取得 → collect() で Vec<String> に
//   - args[0] = バイナリ名, args[1] = 検索文字列, args[2] = ファイルパス
//
// 12.2: ファイルを読み込む
//   - std::fs::read_to_string() で String として読み込み
//
// 12.3: リファクタリング — モジュール性とエラー処理の改善
//   - main の責任を最小限にする
//   - Config 構造体で設定をまとめる
//   - Config::build() で Result を返す（panic ではなく）
//   - run() 関数にロジックを分離
//   - lib.rs にロジックを移動（テスト可能に）
//
// 使い方:
//   cargo run -- <検索文字列> <ファイルパス>
//   cargo run -- the poem.txt

use std::env;
use std::process;

use minigrep::Config;
//  ^^^^^^^^ クレート名（Cargo.toml の [package] name）
// lib.rs の pub な要素を use できる

fn main() {
    // 12.1: コマンドライン引数の収集
    let args: Vec<String> = env::args().collect();

    // 12.3: Config::build で引数を解析（エラーハンドリング付き）
    let config = Config::build(&args).unwrap_or_else(|err| {
        // unwrap_or_else:
        //   Ok → 中身を取り出す（unwrap と同じ）
        //   Err → クロージャを実行
        //   unwrap() と違い、パニックせずにカスタム処理ができる
        println!("Problem parsing arguments: {err}");
        process::exit(1);
        // process::exit(1) でプログラムを即座に終了（終了コード 1 = エラー）
    });

    // 12.3: run() にロジックを委譲
    if let Err(e) = minigrep::run(config) {
        // if let Err(e) = ... :
        //   Ok なら何もしない（戻り値の () は使わない）
        //   Err ならエラーメッセージを表示して終了
        println!("Application error: {e}");
        process::exit(1);
    }
}

// ====================================================================
// リファクタリング前の main（12.2 の状態）— 参考用
// ====================================================================
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let query = &args[1];        // 引数不足で panic（index out of bounds）
//     let file_path = &args[2];    // エラーメッセージが不親切
//
//     let contents = fs::read_to_string(file_path)
//         .expect("Should have been able to read the file");
//                  // ファイル名が表示されない → 原因特定しにくい
//
//     println!("With text:\n{contents}");
// }
//
// 問題点:
//   1. main が引数解析 + ファイル読み込み + 出力を全部やっている
//   2. query と file_path が散らばっている（Config にまとめるべき）
//   3. expect でパニック → ユーザーに不親切なエラーメッセージ
//   4. エラー処理が各所に分散 → 一箇所にまとめるべき
