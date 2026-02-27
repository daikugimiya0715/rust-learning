// ====================================================================
// 13章: 関数型言語の機能 — クロージャ (13.1)
// ====================================================================
//
// クロージャ = 変数に保存したり引数として渡せる「無名関数」
// 関数と違い、定義されたスコープの変数をキャプチャできる

use std::thread;

// ====================================================================
// Tシャツ配布の例 — unwrap_or_else にクロージャを渡す
// ====================================================================

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // ユーザーの好みの色があればそれを、なければ在庫最多の色を配る
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // unwrap_or_else は Option<T> のメソッド
        //   Some(value) → value を返す
        //   None → 引数のクロージャを呼び出して結果を返す
        //
        // クロージャ || self.most_stocked() のポイント:
        //   - 引数なし（|| で始まる）
        //   - self を不変借用でキャプチャ
        //   - 本体は self.most_stocked() の1式だけ
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // --- Tシャツ配布の例 ---
    println!("==== Tシャツ配布の例 ====");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "好み {:?} のユーザーは {:?} をもらった",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "好みなしのユーザーは {:?} をもらった（在庫最多）",
        giveaway2
    );

    // ====================================================================
    // クロージャの型注釈と型推論
    // ====================================================================
    //
    // 関数は常に型注釈が必要だが、クロージャは省略できる
    // コンパイラが使われ方から型を推論してくれる
    //
    // 以下の4つは全て同じ意味:
    //
    //   fn  add_one_v1   (x: u32) -> u32 { x + 1 }   // 関数
    //   let add_one_v2 = |x: u32| -> u32 { x + 1 };   // 型注釈あり
    //   let add_one_v3 = |x|             { x + 1 };    // 型推論
    //   let add_one_v4 = |x|               x + 1  ;    // 中カッコ省略
    //
    // 注意: クロージャは最初の使用時に型が確定する
    //   let example = |x| x;
    //   let s = example(String::from("hello")); // → x: String に確定
    //   let n = example(5);                      // → エラー! String 期待なのに i32

    println!("\n==== クロージャの型推論 ====");
    let add_one = |x: u32| -> u32 { x + 1 };
    println!("add_one(5) = {}", add_one(5));

    // ====================================================================
    // 環境キャプチャ — 3つの方法
    // ====================================================================
    //
    // クロージャは定義されたスコープの変数を3つの方法でキャプチャできる:
    //   1. 不変借用   (&T)     — Fn トレイト
    //   2. 可変借用   (&mut T) — FnMut トレイト
    //   3. 所有権移動 (T)      — FnOnce トレイト
    //
    // コンパイラはクロージャの本体を見て、
    // 必要最小限のキャプチャ方法を自動で選ぶ

    // --- 1. 不変借用 ---
    println!("\n==== 環境キャプチャ: 不変借用 ====");
    let list = vec![1, 2, 3];
    println!("クロージャ定義前: {:?}", list);

    let only_borrows = || println!("クロージャ内: {:?}", list);

    println!("クロージャ呼び出し前: {:?}", list); // 不変借用なので使える
    only_borrows();
    println!("クロージャ呼び出し後: {:?}", list); // まだ使える

    // --- 2. 可変借用 ---
    println!("\n==== 環境キャプチャ: 可変借用 ====");
    let mut list = vec![1, 2, 3];
    println!("クロージャ定義前: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    // ↑ 定義した時点で list への可変借用が始まる
    // println!("{:?}", list); // ← エラー! 可変借用中は不変借用できない
    borrows_mutably();
    // ↑ 呼び出し後、可変借用が終了
    println!("クロージャ呼び出し後: {:?}", list); // [1, 2, 3, 7]

    // --- 3. 所有権移動（move） ---
    println!("\n==== 環境キャプチャ: 所有権移動 (move) ====");
    let list = vec![1, 2, 3];
    println!("スレッド生成前: {:?}", list);

    // move キーワードでクロージャに所有権を移動
    // thread::spawn は新スレッドで実行するので、
    // メインスレッドの list が先にドロップされる可能性がある
    // → move で所有権をクロージャに渡す必要がある
    let handle = thread::spawn(move || {
        println!("スレッド内: {:?}", list);
    });
    // println!("{:?}", list); // ← エラー! 所有権はクロージャに移動済み
    handle.join().unwrap();

    // ====================================================================
    // Fn トレイトの階層
    // ====================================================================
    //
    // クロージャが実装する Fn トレイトは3種類（包含関係あり）:
    //
    //   FnOnce — キャプチャした値の所有権を「消費」できる
    //            全てのクロージャは最低1回は呼べるので、全クロージャが実装
    //
    //   FnMut  — キャプチャした値を変更できるが、消費はしない
    //            複数回呼べる。FnOnce も実装している
    //
    //   Fn     — キャプチャした値を変更しない（不変借用 or キャプチャなし）
    //            FnMut + FnOnce も実装している
    //
    // 階層: Fn ⊂ FnMut ⊂ FnOnce
    //
    // unwrap_or_else の例:
    //   シグネチャ: pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T
    //   FnOnce を要求 → どんなクロージャでも渡せる（1回だけ呼ぶから）

    // ====================================================================
    // sort_by_key — FnMut が必要な例
    // ====================================================================
    //
    // sort_by_key はクロージャを各要素に対して複数回呼ぶ
    // → FnMut を要求する

    println!("\n==== sort_by_key (FnMut の例) ====");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // sort_by_key に渡すクロージャは FnMut を実装する必要がある
    // |r| r.width は r を不変借用するだけ → Fn を実装 → FnMut も実装 → OK
    list.sort_by_key(|r| r.width);
    println!("width でソート: {:#?}", list);

    // --- FnOnce しか実装しないクロージャは sort_by_key に渡せない ---
    //
    // let mut sort_operations = vec![];
    // list.sort_by_key(|r| {
    //     sort_operations.push(String::from("called")); // ← FnOnce: 値をクロージャの外に移動
    //     r.width
    // });
    // ↑ エラー! sort_operations.push で String をクロージャ環境の外に移動するため
    //   FnOnce しか実装できない → sort_by_key は FnMut を要求するのでダメ

    // 解決策: カウンタで数える（可変借用 → FnMut）
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1; // 可変借用 → FnMut を実装
        r.width
    });
    println!("ソート操作回数: {num_sort_operations}");
}
