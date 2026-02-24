// Step 7: ? と Option
// ? は Result だけでなく Option にも使える
// Some なら値を取り出し、None なら早期リターンで None を返す

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn run() {
    println!(
        "\"Hello\\nWorld\" の最初の行の最後の文字: {:?}",
        last_char_of_first_line("Hello\nWorld")
    );
    println!(
        "空文字列の場合: {:?}",
        last_char_of_first_line("")
    );
    println!(
        "\"\\nhi\" の場合: {:?}",
        last_char_of_first_line("\nhi")
    );
}
