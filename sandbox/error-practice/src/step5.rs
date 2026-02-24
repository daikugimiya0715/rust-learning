// Step 5: ? 演算子で簡潔に

use std::fs::{self, File};
use std::io::{self, Read};

// 5-a: ? 演算子で書き直し
// ? は Ok なら値を取り出し、Err なら return Err(e) と同じ
fn read_username_from_file_question() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 5-b: メソッドチェーン版
fn read_username_from_file_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 5-c: fs::read_to_string で最短
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

pub fn run() {
    println!(
        "v1 (?演算子):      {:?}",
        read_username_from_file_question()
    );
    println!("v2 (チェーン):     {:?}", read_username_from_file_chained());
    println!(
        "v3 (read_to_string): {:?}",
        read_username_from_file_shortest()
    );
}
