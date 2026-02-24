// Step 1: ジェネリクスの動機 — コードの重複を排除する
// 同じロジックを型ごとに書くと重複が生まれる
// → ジェネリクスで 1 つの関数にまとめたい（ただしトレイト境界が必要 → Step 7 で解決）

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ジェネリック版 — まだコンパイルできない！
// T に > 演算子を使うには PartialOrd トレイト境界が必要
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {   // エラー: binary operation `>` cannot be applied to type `&T`
//             largest = item;
//         }
//     }
//     largest
// }
// → Step 7 で `fn largest<T: PartialOrd>(list: &[T]) -> &T` として解決する

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("最大の数値: {}", largest_i32(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("最大の文字: {}", largest_char(&char_list));

    println!();
    println!("--- 注目ポイント ---");
    println!("largest_i32 と largest_char のロジックは完全に同じ");
    println!("違いは型シグネチャだけ → ジェネリクスで統一したい");
    println!("ただし T に比較演算を使うにはトレイト境界が必要（Step 7 で解決）");
}
