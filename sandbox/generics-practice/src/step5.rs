// Step 5: トレイトの定義と実装
// トレイト = 型が持つべき振る舞い（メソッド）を定義するインターフェース

pub trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, {} 著 ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn run() {
    let article = NewsArticle {
        headline: String::from("ペンギン、スタンレーカップ優勝！"),
        location: String::from("ピッツバーグ, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("ピッツバーグ・ペンギンズが再び NHL で最高のチームに..."),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知の通り、みなさん"),
        reply: false,
        retweet: false,
    };

    println!("記事の要約: {}", article.summarize());
    println!("ツイートの要約: {}", tweet.summarize());

    println!();
    println!("--- 注目ポイント ---");
    println!("トレイトは共通のインターフェースを定義する");
    println!("各型が自分自身の方法でメソッドを実装する");
    println!("→ ポリモーフィズム（多態性）を実現");
}
