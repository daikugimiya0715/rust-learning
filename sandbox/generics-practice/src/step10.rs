// Step 10: ライフタイムの基本 — ダングリング参照の防止
// ライフタイム = 参照が有効である期間
// コンパイラは参照が無効なメモリを指さないことを保証する

// --- ダングリング参照の例（コンパイルエラーになる）---
// fn dangling() -> &String {
//     let s = String::from("hello");
//     &s  // s はこの関数の終わりで解放される → 参照が無効に！
// }

// --- longest 関数 ---
// 2 つの文字列スライスのうち長い方を返す
// 戻り値のライフタイムはどちらの引数のものか？ → 明示する必要がある
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
// 'a の意味: 戻り値の参照は x と y の両方のライフタイムの「短い方」まで有効

pub fn run() {
    // ケース 1: 両方同じスコープ → OK
    let string1 = String::from("長い文字列です");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("長い方: {}", result);

    // ケース 2: ライフタイムが異なるスコープ → 短い方に合わせる
    let string1 = String::from("長い文字列です");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // string2 はまだ有効 → result も有効
        println!("長い方: {}", result);
    }

    // println!("長い方: {}", result);

    // ケース 3: コンパイルエラーになる例
    // let string1 = String::from("長い文字列です");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("{}", result); // エラー！string2 は解放済み

    println!();
    println!("--- ライフタイム注釈の意味 ---");
    println!("fn longest<'a>(x: &'a str, y: &'a str) -> &'a str");
    println!("  'a は「x と y の両方のライフタイムの短い方」を表す");
    println!("  戻り値はその短い方の期間だけ有効");
    println!("  → ダングリング参照を防ぐ");
}
