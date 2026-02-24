// Step 4: エラーの委譲（propagation）— match 版
// エラーを呼び出し元に返す冗長なパターン

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn run() {
    match read_username_from_file() {
        Ok(name) => println!("ユーザー名: {name}"),
        Err(e) => println!("エラー: {e}"),
    }
}
