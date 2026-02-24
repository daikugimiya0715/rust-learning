// Step 10: 検証のために独自の型を作る — Guess 型
// 型システムで不正な値の存在を防ぐパターン
// 1〜100 の範囲を保証する Guess 構造体を作成

pub struct Guess {
    // フィールドを非公開にして、new() を通さないと作れないようにする
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "Guess の値は 1〜100 でなければなりません。受け取った値: {}",
                value
            );
        }
        Guess { value }
    }

    // ゲッターメソッド（フィールドが非公開なので、これが唯一のアクセス手段）
    pub fn value(&self) -> i32 {
        self.value
    }
}

pub fn run() {
    println!("=== Guess 型: 型システムで値を検証 ===\n");

    // 正常な値で Guess を作成
    let valid_guesses = [1, 50, 100];
    for &v in &valid_guesses {
        let g = Guess::new(v);
        println!("Guess::new({}) → value = {}", v, g.value());
    }

    // フィールドが非公開なので、直接アクセスはコンパイルエラーになる
    // let g = Guess { value: 50 };  // エラー: field `value` is private
    // g.value = 200;                // エラー: field `value` is private
    println!("\n→ フィールドは非公開なので Guess::new() を通すしかない");
    println!("→ つまり Guess が存在する時点で 1〜100 が保証される");

    // 範囲外の値（コメントを外すと panic する）
    // Guess::new(0);   // panic: 1未満
    // Guess::new(101); // panic: 100超

    // 実用例: Guess を受け取る関数は範囲チェック不要
    fn print_guess(guess: &Guess) {
        // guess.value() は必ず 1〜100 なので、ここでチェックする必要がない
        println!("あなたの推測: {}", guess.value());
    }

    let my_guess = Guess::new(42);
    print_guess(&my_guess);

    println!("\n--- まとめ ---");
    println!("独自型で値を検証するパターン:");
    println!("  1. フィールドを非公開にする");
    println!("  2. new() で検証し、不正な値なら panic!");
    println!("  3. ゲッター (value()) で読み取り専用アクセスを提供");
    println!("  4. 型が存在する = 値が有効 → 以降のチェックが不要に");
}
