// Step 1: Result の基本 — File::open と match
// File::open の戻り値は Result<File, io::Error>
// match で Ok / Err を分岐して処理する

use std::fs::File;

pub fn run() {
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("やった");
            file
        }
        Err(error) => panic!("ファイルを開けませんでした: {error:?}"),
    };
}
