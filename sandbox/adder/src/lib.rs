// ====================================================================
// 11章: テストの記述（学習用ライブラリ）
// ====================================================================

// --- テスト対象のコード ---

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    #[allow(dead_code)]
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "予想値は1以上でなければなりませんが、{}でした。",
                value
            );
        } else if value > 100 {
            panic!(
                "予想値は100以下でなければなりませんが、{}でした。",
                value
            );
        }
        Guess { value }
    }
}

// 非公開関数 — ユニットテストからはテストできる（11.3）
#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// ====================================================================
// 11.1: テストの書き方
// ====================================================================
//
// テスト関数の3ステップ:
//   1. 必要なデータや状態をセットアップする
//   2. テスト対象のコードを実行する
//   3. 結果が期待通りかアサートする
//
// 主なアサートマクロ:
//   assert!(条件)           — 条件が true か検証
//   assert_eq!(左, 右)      — 2値が等しいか検証（失敗時に両方の値を表示）
//   assert_ne!(左, 右)      — 2値が異なるか検証
//   ※ assert_eq!/assert_ne! は PartialEq + Debug トレイトが必要
//
// cargo test の実行の流れ:
//   cargo test
//     → #[cfg(test)] のコードをコンパイル
//     → #[test] の付いた関数をすべて実行
//     → パニックしなければ pass、パニックすれば fail

// ====================================================================
// 11.2: テストの実行方法（コマンドラインオプション）
// ====================================================================
//
// cargo test                          — 全テスト実行（並列）
// cargo test -- --test-threads=1      — シングルスレッドで実行（順次）
// cargo test -- --show-output         — 成功テストの stdout も表示
// cargo test it_works                 — 名前が一致するテストだけ実行
// cargo test add                      — 名前に "add" を含むテスト全部
// cargo test -- --ignored             — #[ignore] のテストだけ実行
// cargo test -- --include-ignored     — 全テスト（ignore 含む）実行

// ====================================================================
// 11.3: テストの体系化
// ====================================================================
//
// ユニットテスト:
//   - src/ 内に #[cfg(test)] mod tests で書く
//   - 非公開関数もテストできる（子モジュールから use super::* で親にアクセス）
//   - cargo test 時のみコンパイルされる
//
// 結合テスト:
//   - プロジェクトルートの tests/ ディレクトリに置く
//   - 各ファイルが独立したクレートとしてコンパイルされる
//   - pub な API のみテスト可能（外部利用者と同じ視点）
//   - 共通ヘルパーは tests/common/mod.rs に置く

#[cfg(test)]
mod tests {
    use super::*;

    // ----------------------------------------------------------------
    // 11.1 基本: #[test] と assert_eq!
    // ----------------------------------------------------------------

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // ----------------------------------------------------------------
    // 11.1 assert! — bool 値のテスト
    // ----------------------------------------------------------------

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger)); // ! で否定
    }

    // ----------------------------------------------------------------
    // 11.1 assert_eq! / assert_ne!
    // ----------------------------------------------------------------

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn it_does_not_add_three() {
        assert_ne!(5, add_two(2)); // add_two(2) は 4 であり 5 ではない
    }

    // ----------------------------------------------------------------
    // 11.1 カスタム失敗メッセージ
    // ----------------------------------------------------------------
    // assert! の第2引数以降は format! と同じ書式で失敗メッセージを書ける

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            // 失敗時のカスタムメッセージ（デバッグに役立つ）
            "greeting が名前を含んでいません。値は `{}` でした",
            result
        );
    }

    // ----------------------------------------------------------------
    // 11.1 #[should_panic] — パニックすることをテスト
    // ----------------------------------------------------------------

    // expected を付けないと、どんな理由のパニックでも pass になる
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail.");
    }

    // expected を付けると、パニックメッセージに含まれる文字列を検証できる
    #[test]
    #[should_panic(expected = "予想値は100以下でなければなりません")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "予想値は1以上でなければなりません")]
    fn less_than_1() {
        Guess::new(0);
    }

    // ----------------------------------------------------------------
    // 11.1 Result<T, E> を返すテスト
    // ----------------------------------------------------------------
    // パニックの代わりに Err を返すとテスト失敗になる
    // → ? 演算子が使えるので、エラー処理のテストに便利
    // ※ #[should_panic] とは併用できない

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 が 4 になりませんでした"))
        }
    }

    // ----------------------------------------------------------------
    // 11.2 #[ignore] — 時間のかかるテストをスキップ
    // ----------------------------------------------------------------
    // cargo test          → スキップされる
    // cargo test -- --ignored → これだけ実行される

    #[test]
    #[ignore]
    fn expensive_test() {
        // 実行に時間のかかる処理をシミュレート
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        assert_eq!(sum, 499_999_500_000_i64);
    }

    // ----------------------------------------------------------------
    // 11.3 非公開関数のテスト
    // ----------------------------------------------------------------
    // use super::* で親モジュールの非公開関数にもアクセスできる

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
