// Step 4: 単相化(monomorphization) — ランタイムコストゼロ
// コンパイラがジェネリクスを具体型ごとのコードに展開する
// → 実行時にはジェネリクスのオーバーヘッドはゼロ

pub fn run() {
    // 我々が書くコード
    let integer = Some(5);
    let float = Some(5.0);

    println!("integer: {:?}", integer);
    println!("float: {:?}", float);

    println!();
    println!("--- 単相化の仕組み ---");
    println!("コンパイラは Option<T> を使っている箇所を調べ、");
    println!("使われている具体型ごとに専用のコードを生成する。");
    println!();
    println!("例: 以下のような具体型が内部的に作られる");
    println!();
    println!("  // Option<i32> 用");
    println!("  enum Option_i32 {{");
    println!("      Some(i32),");
    println!("      None,");
    println!("  }}");
    println!();
    println!("  // Option<f64> 用");
    println!("  enum Option_f64 {{");
    println!("      Some(f64),");
    println!("      None,");
    println!("  }}");
    println!();
    println!("つまり実行時にはジェネリクスは存在しない。");
    println!("手動で具体型ごとに書いた場合と同じ速度で動く。");
    println!("これが Rust の「ゼロコスト抽象化」の一例。");
}
