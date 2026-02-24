// Step 9: derive で自動実装できるトレイトまとめ
// #[derive(...)] は「手続き的マクロ（derive マクロ）」の一種
// コンパイラが構造体/列挙型の定義を読み取り、トレイトの impl を自動生成する
//
// Rust のマクロは大きく 2 種類:
//   1. 宣言的マクロ (macro_rules!) — println!, vec! など
//   2. 手続き的マクロ — さらに 3 種類:
//      - derive マクロ   — #[derive(Debug, Clone, ...)]
//      - 属性マクロ      — #[route(GET, "/")]
//      - 関数風マクロ    — sql!(SELECT * FROM ...)

// よく使う derive トレイト一覧:
// Debug       — {:?} でデバッグ出力
// Clone       — .clone() でディープコピー
// Copy        — 代入時に暗黙コピー（Clone が前提）
// PartialEq   — == / != で比較
// Eq          — 完全な等価性（PartialEq が前提、f32/f64 は不可）
// PartialOrd  — < > <= >= で比較
// Ord         — 完全な順序（Eq + PartialOrd が前提）
// Hash        — ハッシュ値を計算（HashMap のキーに使える）
// Default     — デフォルト値を生成

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct User {
    name: String,
    age: u32,
}

// Copy は String を含む型には使えない
// Copy が使える例: 全フィールドが Copy を実装している型
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

pub fn run() {
    // Debug
    let user = User {
        name: String::from("太郎"),
        age: 25,
    };
    println!("Debug: {:?}", user);

    // Clone
    let user2 = user.clone();
    println!("Clone: {:?}", user2);

    // PartialEq
    println!("user == user2: {}", user == user2);

    // Ord（比較 — name → age の順でフィールドを比較）
    let user3 = User {
        name: String::from("太郎"),
        age: 30,
    };
    println!("user < user3: {}", user < user3);

    // Default
    let default_user = User::default();
    println!("Default: {:?}", default_user); // User { name: "", age: 0 }

    println!();

    // Copy — 代入しても元の変数が使える
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy なのでムーブではなくコピー
    println!("p1: {:?}, p2: {:?}", p1, p2); // 両方使える！

    // String を持つ User は Copy ではないのでムーブする
    // let u1 = User { name: String::from("A"), age: 1 };
    // let u2 = u1; // ムーブ
    // println!("{:?}", u1); // エラー！u1 はもう使えない

    println!();
    println!("--- derive トレイト早見表 ---");
    println!("Debug      → {{:?}} でデバッグ出力");
    println!("Clone      → .clone() でディープコピー");
    println!("Copy       → 暗黙コピー（全フィールドが Copy の型のみ）");
    println!("PartialEq  → == / !=");
    println!("Eq         → 完全等価（NaN がない型）");
    println!("PartialOrd → < > <= >=");
    println!("Ord        → 完全順序（Eq が前提）");
    println!("Hash       → HashMap のキーに使える");
    println!("Default    → 型のデフォルト値");
}
