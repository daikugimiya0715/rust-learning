// Step 3: unwrap と expect
// unwrap(): Ok なら値を返し、Err なら panic!
// expect(): unwrap と同じだが、panic 時にカスタムメッセージを表示
// 本番コードでは expect が推奨（なぜ失敗したかの文脈がわかるため）

use std::fs::File;

pub fn run() {
    let _f = File::open("hello.txt").unwrap();
    // let _f = File::open("hello.txt").expect("hello.txt を開けるはずです");
}
