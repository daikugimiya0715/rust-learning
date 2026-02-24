// Step 6: ? を main で使う
// main() -> Result<(), Box<dyn Error>> とすれば main でも ? が使える
// Box<dyn Error> は「あらゆるエラー型」を意味するトレイトオブジェクト

use std::error::Error;
use std::fs::File;

pub fn run() -> Result<(), Box<dyn Error>> {
    let _greeting_file = File::open("hello.txt")?;
    println!("hello.txt を開けました！");
    Ok(())
}
