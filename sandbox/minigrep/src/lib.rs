// ====================================================================
// 12.3: lib.rs へのロジック分離
// ====================================================================
//
// バイナリクレート(main.rs) とライブラリクレート(lib.rs) を分ける理由:
//   - main 関数は直接テストできない
//   - ロジックを lib.rs に移せば、#[cfg(test)] mod tests でテスト可能
//   - 責任の分離: main.rs は「設定 → 実行 → エラー処理」のみ
//
// main.rs の責任（最小限に保つ）:
//   1. コマンドライン引数の解析ロジックを呼ぶ
//   2. 他の設定を行う
//   3. lib.rs の run() を呼ぶ
//   4. run() がエラーを返したら処理する

use std::error::Error;
use std::fs;

// ====================================================================
// Config 構造体
// ====================================================================
//
// リファクタリングの進化:
//
// 【Step 1】まず関数に抽出（タプルを返す）
//   fn parse_config(args: &[String]) -> (&str, &str) {
//       let query = &args[1];
//       let file_path = &args[2];
//       (query, file_path)
//   }
//   → 問題: タプルだと query と file_path の意味が不明瞭
//
// 【Step 2】Config 構造体にまとめる
//   struct Config { query: String, file_path: String }
//   fn parse_config(args: &[String]) -> Config { ... }
//   → 問題: parse_config は Config に紐づくべき → コンストラクタにする
//
// 【Step 3】Config::new にする
//   impl Config {
//       fn new(args: &[String]) -> Config { ... }
//   }
//   → 問題: 引数不足で panic! するのはユーザー体験が悪い
//
// 【Step 4（最終形）】Config::build で Result を返す
//   → new ではなく build という名前にする慣習
//     （new は「失敗しない」という期待があるため）

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //                                         ^^^^^^^^^^^^^^
        // &'static str = 文字列リテラルのライフタイム（10章で学んだ）
        // エラーメッセージは固定文字列なので 'static でOK

        if args.len() < 3 {
            return Err("not enough arguments");
            //     ^^^ panic! ではなく Err を返す
            //     → 呼び出し側がエラーを適切に処理できる
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        // clone() を使う理由:
        //   args は main が所有している Vec<String>
        //   Config が独自に String を所有すれば、ライフタイム管理が単純になる
        //   小さな文字列の clone はパフォーマンスへの影響が軽微

        Ok(Config { query, file_path })
    }
}

// ====================================================================
// run 関数 — プログラムのメインロジック
// ====================================================================
//
// リファクタリング前（main に直書き）:
//   let contents = fs::read_to_string(file_path)
//       .expect("Should have been able to read the file");
//   println!("With text:\n{contents}");
//
// リファクタリング後:
//   - 戻り値を Result にしてエラーを呼び出し側に委譲
//   - expect → ? 演算子（9章で学んだ）

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //                         ^^^^^^^^^^^^^^^^^^^^^^^^
    // Result<(), Box<dyn Error>> の意味:
    //   Ok の場合 : () (ユニット型 = 何も返さない)
    //   Err の場合: Box<dyn Error> (Error トレイトを実装する任意の型)
    //
    // Box<dyn Error> = トレイトオブジェクト（17章で詳しく学ぶ）
    //   今は「あらゆる種類のエラーを返せる」と理解すればOK

    let contents = fs::read_to_string(config.file_path)?;
    //                                                 ^
    // ? 演算子: エラーなら即座に Err を返す（9章で学んだ）
    // expect と違い、パニックせずにエラーを伝播する

    println!("With text:\n{contents}");

    Ok(())
}

// ====================================================================
// テスト（12.4 以降で追加予定）
// ====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_success() {
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::build(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("program")];
        let result = Config::build(&args);
        assert!(result.is_err());
    }
}
