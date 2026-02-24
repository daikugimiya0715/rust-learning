// Step 7: トレイト境界 — Step 1 の問題を解決
// ジェネリック関数の型パラメータにトレイト境界を付けることで、
// 特定の機能を持つ型だけを受け入れる

use std::fmt::Display;

// Step 1 で書けなかったジェネリック版 largest が書ける！
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// --- 3 つのトレイト境界構文 ---

// 構文 1: impl Trait（引数の位置）— シンプルな場合に便利
fn notify_impl(item: &impl Display) {
    println!("[impl Trait] 速報！ {}", item);
}

// 構文 2: <T: Trait>（トレイト境界構文）— 型パラメータを明示
fn notify_bound<T: Display>(item: &T) {
    println!("[Trait Bound] 速報！ {}", item);
}

// 構文 3: where 句 — 境界が多いときに見やすい
fn notify_where<T>(item: &T)
where
    T: Display,
{
    println!("[where]   速報！ {}", item);
}

// 複数のトレイト境界: T は Display かつ PartialOrd を実装していること
fn largest_and_display<T: Display + PartialOrd>(list: &[T]) -> &T {
    let result = largest(list);
    println!("最大値: {}", result);
    result
}

pub fn run() {
    // Step 1 の問題が解決！
    let number_list = vec![34, 50, 25, 100, 65];
    println!("最大の数値: {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("最大の文字: {}", largest(&char_list));

    println!();

    // 3 つの構文はすべて同じ意味
    let message = String::from("大事なニュース");
    notify_impl(&message);
    notify_bound(&message);
    notify_where(&message);

    println!();

    // 複数境界
    let numbers = vec![10, 20, 30, 5, 15];
    largest_and_display(&numbers);

    println!();
    println!("--- 注目ポイント ---");
    println!("impl Trait   → 短くて簡潔（糖衣構文）");
    println!("<T: Trait>   → 同じ型を複数箇所で使うときに必須");
    println!("where T: ... → 境界が多いときに読みやすい");
    println!("T: A + B     → 複数のトレイト境界を同時に要求");
}
