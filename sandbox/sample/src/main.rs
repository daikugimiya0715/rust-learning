// ========================================
// RustOwl 動作確認用サンプル
// 保存後、変数や関数呼び出しにカーソルを合わせると
// 所有権・借用・ライフタイムが色付き下線で表示される
// ========================================

// --- 1. 所有権のムーブ (オレンジの下線が見える) ---
fn take_ownership(s: String) {
    println!("所有権を受け取った: {}", s);
    // この関数が終わるとsはドロップされる
}

// --- 2. 不変借用 (青の下線が見える) ---
fn borrow_immutably(s: &String) {
    println!("不変借用: {}", s);
}

// --- 3. 可変借用 (紫の下線が見える) ---
fn borrow_mutably(s: &mut String) {
    s.push_str(" + 変更済み");
    println!("可変借用: {}", s);
}

// --- 4. ライフタイム注釈付き関数 (ライフタイムの範囲が見える) ---
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // ======== ムーブの確認 ========
    let s1 = String::from("hello");
    let s2 = s1; // s1 → s2 へムーブ (s1はここで無効になる)
    println!("s2: {}", s2);
    // println!("{}", s1); // コンパイルエラー: s1はムーブ済み

    // 関数にムーブ
    let s3 = String::from("world");
    take_ownership(s3); // s3の所有権が関数に移動
    // println!("{}", s3); // コンパイルエラー: s3はムーブ済み

    // ======== 不変借用の確認 ========
    let s4 = String::from("不変借用テスト");
    let r1 = &s4; // 不変参照
    let r2 = &s4; // 複数の不変参照はOK
    borrow_immutably(r1);
    borrow_immutably(r2);
    println!("元の値も使える: {}", s4);

    // ======== 可変借用の確認 ========
    let mut s5 = String::from("可変借用テスト");
    borrow_mutably(&mut s5); // 可変借用
    println!("変更後: {}", s5);

    // ======== ライフタイムの確認 ========
    let result;
    let string1 = String::from("long string");
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("longest: {}", result);
    }
    // resultをここで使うとコンパイルエラーになる
    // println!("{}", result);
    // (string2のライフタイムが終わっているため)

    // ======== Clone vs Move ========
    let original = String::from("クローン元");
    let cloned = original.clone(); // クローン(ムーブではない)
    println!("original: {}, cloned: {}", original, cloned);

    // ======== スコープとドロップ ========
    let outer = String::from("外側");
    {
        let inner = String::from("内側");
        println!("{} と {}", outer, inner);
        // innerはここでドロップ
    }
    println!("outerはまだ使える: {}", outer);
}
