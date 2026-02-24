// Step 8: トレイトを返す / 条件付き実装
// -> impl Trait で戻り値にトレイトを使う
// ブランケット実装の概念

use std::fmt::Display;

trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct NewsArticle {
    headline: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("記事: {}", self.headline)
    }
}

// 戻り値に impl Trait を使う
// 注意: 1 つの具体型のみ返せる（条件分岐で別の型を返すのは不可）
fn create_tweet() -> impl Summary {
    Tweet {
        username: String::from("bot"),
        content: String::from("自動生成ツイート"),
    }
}

// これはコンパイルエラーになる:
// fn create_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         Tweet { username: String::from("a"), content: String::from("b") }
//     } else {
//         NewsArticle { headline: String::from("c") }  // エラー！別の型
//     }
// }

// --- ブランケット実装の例 ---
// Display を実装している全ての型に自動で greet() を提供
trait Greet {
    fn greet(&self) -> String;
}

impl<T: Display> Greet for T {
    fn greet(&self) -> String {
        format!("こんにちは、{} さん！", self)
    }
}

pub fn run() {
    // impl Trait を返す関数
    let tweet = create_tweet();
    println!("{}", tweet.summarize());

    println!();
    println!("--- ブランケット実装 ---");

    // String は Display を実装している → 自動的に Greet も使える
    let name = String::from("太郎");
    println!("{}", name.greet());

    // i32 も Display を実装している → Greet が使える
    let number = 42;
    println!("{}", number.greet());

    // &str も Display を実装している → Greet が使える
    println!("{}", "花子".greet());

    println!();
    println!("--- 注目ポイント ---");
    println!("-> impl Trait は 1 つの具体型しか返せない");
    println!("（複数の型を条件分岐で返したい場合は trait object が必要 → 後の章で）");
    println!();
    println!("ブランケット実装: impl<T: TraitA> TraitB for T");
    println!("→ TraitA を実装する全ての型に TraitB を自動実装");
    println!("標準ライブラリの例: Display を実装 → ToString が自動的に使える");
}
