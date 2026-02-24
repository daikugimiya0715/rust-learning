// Step 12: 'static と全要素の統合
// 'static ライフタイムとジェネリクス + トレイト + ライフタイムの組み合わせ

use std::fmt::Display;

// --- 'static ライフタイム ---
// プログラム全体の期間にわたって有効な参照
// 文字列リテラルはすべて 'static
fn get_static_str() -> &'static str {
    "私は 'static ライフタイムを持つ文字列リテラルです"
    // 文字列リテラルはバイナリに埋め込まれるため、プログラム終了まで有効
}

// --- 全要素の統合 ---
// ジェネリクス + トレイト境界 + ライフタイム を 1 つの関数に
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("お知らせ: {}", ann);
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    // 'static ライフタイム
    let s: &'static str = "文字列リテラルは 'static";
    println!("{}", s);
    println!("{}", get_static_str());

    println!();

    // 'static の注意点
    println!("--- 'static の注意 ---");
    println!("エラーメッセージで 'static を要求されることがあるが、");
    println!("本当に 'static が必要かよく考えること。");
    println!("多くの場合、ダングリング参照やライフタイムの不一致が原因。");

    println!();

    // 全要素の統合
    println!("--- ジェネリクス + トレイト + ライフタイム の統合 ---");
    let string1 = String::from("長い文字列です");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_with_an_announcement(
            string1.as_str(),
            string2.as_str(),
            "比較を行います",
        );
        println!("長い方: {}", result);
    }

    println!();
    println!("fn longest_with_an_announcement<'a, T>(");
    println!("    x: &'a str,");
    println!("    y: &'a str,");
    println!("    ann: T,");
    println!(") -> &'a str");
    println!("where");
    println!("    T: Display,");
    println!();
    println!("この関数は 3 つの概念を組み合わせている:");
    println!("  'a        → ライフタイムパラメータ（参照の有効期間）");
    println!("  T         → ジェネリック型パラメータ");
    println!("  T: Display → トレイト境界（T は Display を実装していること）");
    println!();
    println!("10章のまとめ:");
    println!("  ジェネリクス → コードの重複を排除（コンパイル時に単相化）");
    println!("  トレイト     → 共有された振る舞いを定義");
    println!("  ライフタイム → 参照の有効期間を保証し、ダングリング参照を防ぐ");
}
