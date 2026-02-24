// Step 11: ライフタイム省略規則と構造体のライフタイム
// コンパイラが自動的にライフタイムを推論してくれるルール（省略規則）がある

// --- 3 つのライフタイム省略規則 ---
// 規則 1: 各参照引数にそれぞれ固有のライフタイムが割り当てられる
//   fn foo(x: &str)         → fn foo<'a>(x: &'a str)
//   fn foo(x: &str, y: &str) → fn foo<'a, 'b>(x: &'a str, y: &'b str)
//
// 規則 2: 参照引数が 1 つだけなら、戻り値にもそのライフタイムが付く
//   fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str
//
// 規則 3: メソッド（&self / &mut self がある）なら、戻り値に self のライフタイムが付く
//   fn foo(&self, x: &str) -> &str → fn foo<'a, 'b>(&'a self, x: &'b str) -> &'a str

// 省略規則のおかげでライフタイムを書かなくてよい例
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
// コンパイラが自動で fn first_word<'a>(s: &'a str) -> &'a str と推論

// --- 構造体にライフタイムを持たせる ---
// 構造体が参照を持つ場合、ライフタイム注釈が必須
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// 構造体のメソッドにもライフタイムが必要（ただし規則 3 で省略可能なことが多い）
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 規則 3 により、戻り値は &self のライフタイムを受け取る
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("お知らせ: {}", announcement);
        self.part
    }
}

pub fn run() {
    // 省略規則の例
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("最初の単語: {}", word);

    println!();

    // 構造体にライフタイムを持たせる
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        first_sentence = &novel[..i];
    }
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    println!("抜粋: {:?}", excerpt);
    println!("レベル: {}", excerpt.level());
    println!("{}", excerpt.announce_and_return_part("注目！"));

    // 構造体の参照は、参照先のデータより長く生存できない
    // let excerpt;
    // {
    //     let novel = String::from("短い文");
    //     excerpt = ImportantExcerpt { part: &novel };  // novel はスコープ終了で解放
    // }
    // println!("{:?}", excerpt); // エラー！novel は解放済み

    println!();
    println!("--- ライフタイム省略規則まとめ ---");
    println!("規則 1: 各引数に固有のライフタイムを割り当てる");
    println!("規則 2: 参照引数が 1 つ → 戻り値にもそのライフタイム");
    println!("規則 3: &self がある → 戻り値に self のライフタイム");
    println!("これらで決まらない場合のみ、手動でライフタイム注釈を書く");
}
