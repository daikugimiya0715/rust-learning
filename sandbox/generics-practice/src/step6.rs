// Step 6: デフォルト実装とオーバーライド
// トレイトのメソッドにデフォルトの本体を書ける
// 各型はデフォルトをそのまま使うか、オーバーライドできる

trait Summary {
    fn summarize_author(&self) -> String;

    // デフォルト実装: summarize_author() を呼び出す
    fn summarize(&self) -> String {
        format!("({}さんの記事をもっと読む...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    author: String,
}

// summarize_author のみ実装、summarize はデフォルトを使う
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    username: String,
    content: String,
}

// summarize_author を実装し、summarize もオーバーライド
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}

pub fn run() {
    let article = NewsArticle {
        headline: String::from("ペンギン、スタンレーカップ優勝！"),
        author: String::from("Iceburgh"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知の通り"),
    };

    // NewsArticle はデフォルト実装を使う
    println!("記事: {}", article.summarize());
    // Tweet はオーバーライドした実装を使う
    println!("ツイート: {}", tweet.summarize());

    println!();
    println!("--- 注目ポイント ---");
    println!("デフォルト実装があれば、各型は必要な部分だけ実装すればよい");
    println!("デフォルト実装は同じトレイト内の他のメソッドを呼び出せる");
    println!("（そのメソッドにデフォルト実装がなくても OK）");
}
