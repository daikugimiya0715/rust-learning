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
    // 13.3: &[String] → impl Iterator<Item = String> に変更
    //
    // 改善点:
    //   - clone() が不要になった（イテレータが String の所有権を直接渡す）
    //   - インデックスアクセス（args[1], args[2]）→ next() で順番に取得
    //   - 引数の長さチェックも不要（next() が None を返せばエラー）
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 最初の要素はプログラム名なのでスキップ
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // 12.5: 環境変数 IGNORE_CASE が設定されているか確認
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
// ライフタイム注釈が必要な理由:
//   戻り値 Vec<&str> の中身は contents のスライスを指している
//   → contents のライフタイム 'a を戻り値にも付ける必要がある
//
// 13.3: for ループ → イテレータチェーンに書き換え
//   lines().filter().collect() で同じ処理をより宣言的に

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// ====================================================================
// 12.5: 大文字小文字を区別しない検索
// ====================================================================
//
// 13.3: こちらもイテレータチェーンに書き換え

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// ====================================================================
// 13.4: ゼロコスト抽象化（Zero-Cost Abstractions）
// ====================================================================
//
// Rust のイテレータは「ゼロコスト抽象化」の代表例:
//   - 高レベルな抽象（map, filter, collect）を使っても
//     手書きの低レベルループと同等の機械語にコンパイルされる
//   - 「使わない機能にはコストを払わない」「使う機能にはこれ以上ないほど最適化される」
//
// 例: オーディオデコーダのベンチマーク
//   - イテレータ版と手書きループ版で同じ実行速度
//   - コンパイラがイテレータチェーンを「展開（unroll）」して最適化
//   - ループの境界チェックも不要と判断し除去する
//
// → 安心して高レベルな書き方ができる（パフォーマンスのペナルティなし）

// ====================================================================
// テスト
// ====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Config のテスト ---

    #[test]
    fn config_build_success() {
        // 13.3: イテレータを直接渡す
        let args = vec![
            String::from("program"),
            String::from("query"),
            String::from("file.txt"),
        ];
        let config = Config::build(args.into_iter()).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.file_path, "file.txt");
    }

    #[test]
    fn config_build_not_enough_args() {
        let args = vec![String::from("program")];
        let result = Config::build(args.into_iter());
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
