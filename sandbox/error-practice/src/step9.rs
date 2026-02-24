// Step 9: エラー処理のガイドライン — panic! vs Result
// panic! すべきとき: 不正な状態（contract 違反）、回復不能
// Result を返すべきとき: 失敗が予測可能、呼び出し側に判断を委ねたい

use std::num::ParseIntError;

// ── panic! すべき例: 不正な状態（contract 違反）──
// 前提条件が破られたとき → プログラムのバグなので panic! で即座に通知
fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        panic!("空のスライスの平均は計算できません（呼び出し側のバグ）");
    }
    let sum: f64 = numbers.iter().sum();
    sum / numbers.len() as f64
}

// ── Result を返すべき例: 失敗が予測可能 ──
// ユーザー入力のパースは失敗して当然 → 呼び出し側に判断を委ねる
fn parse_age(input: &str) -> Result<u8, String> {
    let age: u8 = input
        .parse::<u8>()
        .map_err(|e: ParseIntError| format!("年齢のパースに失敗: {}", e))?;

    if age > 150 {
        return Err(format!("年齢 {} は現実的ではありません", age));
    }
    Ok(age)
}

// ── panic! すべき例: 到達不能な状態 ──
// match のアームで論理的に到達し得ないとき
fn direction_from_char(c: char) -> &'static str {
    match c {
        'N' => "北",
        'S' => "南",
        'E' => "東",
        'W' => "西",
        _ => panic!("無効な方角文字: '{}' (N/S/E/W のみ有効)", c),
    }
}

// ── Result を返すべき例: 外部リソースへのアクセス ──
fn read_config_value(config: &std::collections::HashMap<String, String>, key: &str) -> Result<String, String> {
    config
        .get(key)
        .cloned()
        .ok_or_else(|| format!("設定キー '{}' が見つかりません", key))
}

pub fn run() {
    println!("=== panic! すべき場面 ===\n");

    // 正常ケース
    let nums = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("平均: {}", calculate_average(&nums));

    // contract 違反（コメントを外すと panic する）
    // calculate_average(&[]);

    // 方角変換
    for c in ['N', 'S', 'E', 'W'] {
        println!("'{}' → {}", c, direction_from_char(c));
    }
    // 無効な方角（コメントを外すと panic する）
    // direction_from_char('X');

    println!("\n=== Result を返すべき場面 ===\n");

    // ユーザー入力のパース — 成功と失敗の両方を体験
    let test_inputs = ["25", "abc", "200", "0"];
    for input in test_inputs {
        match parse_age(input) {
            Ok(age) => println!("入力 \"{}\" → 年齢: {}", input, age),
            Err(e) => println!("入力 \"{}\" → エラー: {}", input, e),
        }
    }

    // 設定値の取得
    let mut config = std::collections::HashMap::new();
    config.insert("host".to_string(), "localhost".to_string());
    config.insert("port".to_string(), "8080".to_string());

    for key in ["host", "port", "database"] {
        match read_config_value(&config, key) {
            Ok(val) => println!("config[{}] = {}", key, val),
            Err(e) => println!("config エラー: {}", e),
        }
    }

    println!("\n--- まとめ ---");
    println!("panic! すべきとき:");
    println!("  - 不正な状態（contract 違反、プログラムのバグ）");
    println!("  - 回復不能な論理エラー");
    println!("Result を返すべきとき:");
    println!("  - 失敗が予測可能（ユーザー入力、外部リソース）");
    println!("  - 呼び出し側に判断を委ねたい場合");
}
