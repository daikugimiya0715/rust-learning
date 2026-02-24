// Step 2: ErrorKind で複数のエラーを区別
// error.kind() で ErrorKind を取得し、エラーの種類ごとに処理を分けられる
// NotFound → ファイルを作成、その他 → panic!

use std::fs::File;
use std::io::ErrorKind;

pub fn run() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ファイルの作成に失敗しました: {e:?}"),
            },
            other_error => {
                panic!("ファイルを開けませんでした: {other_error:?}");
            }
        },
    };

    println!("hello.txt を開く or 作成しました！");
}
