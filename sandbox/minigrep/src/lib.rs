// ====================================================================
// 12章: minigrep — ライブラリクレート (lib.rs)
// ====================================================================
//
// 12.3: main.rs とlib.rs の責任分離
//   - main.rs: 設定 → 実行 → エラー処理（薄く保つ）
//   - lib.rs: ロジック本体（テスト可能）
//
// 12.4: TDD で search 関数を開発
// 12.5: 環境変数で大文字小文字の区別を制御
// 12.6: エラーメッセージを stderr に出力（main.rs 側）

use std::env;
use std::error::Error;
use std::fs;

// ====================================================================
// Config 構造体
// ====================================================================
//
// リファクタリングの進化（12.3）:
//   【Step 1】parse_config() でタプルを返す → 意味が不明瞭
//   【Step 2】Config 構造体にまとめる → parse_config は Config に紐づくべき
//   【Step 3】Config::new にする → panic するのはユーザー体験が悪い
//   【Step 4】Config::build で Result を返す（最終形）

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool, // 12.5: 大文字小文字を無視するか
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        // 12.5: 環境変数 IGNORE_CASE が設定されているか確認
        // env::var() は Result を返す
        //   Ok(値) → 環境変数が存在する
        //   Err    → 環境変数が存在しない
        // .is_ok() で bool に変換（値の中身は気にしない）
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// ====================================================================
// run 関数 — プログラムのメインロジック
// ====================================================================

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // 12.5: ignore_case に応じて検索関数を切り替え
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// ====================================================================
// 12.4: search 関数 — TDD で開発
// ====================================================================
//
// TDD の手順:
//   1. 失敗するテストを書く（one_result テスト）
//   2. テストが通る最小限のコードを書く
//   3. リファクタリング
//   4. 繰り返し
//
// ライフタイム注釈が必要な理由:
//   戻り値 Vec<&str> の中身は contents のスライスを指している
//   → contents のライフタイム 'a を戻り値にも付ける必要がある
//   query は結果に含まれないのでライフタイム不要

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // contents.lines() — 各行のイテレータを返す
    // line.contains(query) — その行に query が含まれるか
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// ====================================================================
// 12.5: 大文字小文字を区別しない検索
// ====================================================================
//
// to_lowercase() で query と line の両方を小文字に変換してから比較
// 注意: to_lowercase() は新しい String を返す（&str ではない）
//   → contains に渡すときは &query のように参照にする

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // String 型になる

    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            //                          ^^^^^^ String → &str に変換
            results.push(line); // 元の行（大文字小文字そのまま）を返す
        }
    }

    results
}

// ====================================================================
// テスト
// ====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Config のテスト ---

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

    // --- 12.4: search 関数のテスト（大文字小文字を区別） ---

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        // "duct" は "productive." にマッチ
        // "Duct" は大文字なのでマッチしない
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // --- 12.5: search_case_insensitive のテスト ---

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        // "rUsT" は大文字小文字無視で "Rust:" と "Trust me." にマッチ
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
