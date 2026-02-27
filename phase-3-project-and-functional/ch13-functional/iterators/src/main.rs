// ====================================================================
// 13章: 関数型言語の機能 — イテレータ (13.2)
// ====================================================================
//
// イテレータ = 一連の要素を順番に処理するパターン
// Rust のイテレータは「怠惰（lazy）」— 消費するまで何もしない

fn main() {
    // ====================================================================
    // Iterator トレイトの基本
    // ====================================================================
    //
    // pub trait Iterator {
    //     type Item;                       // 関連型（イテレータが返す要素の型）
    //     fn next(&mut self) -> Option<Self::Item>;  // 唯一の必須メソッド
    //     // ... 多数のデフォルトメソッド（sum, map, filter, etc.）
    // }
    //
    // next() は要素を1つずつ返す:
    //   Some(value) → まだ要素がある
    //   None        → 終端に達した

    println!("==== next() メソッド ====");
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // mut が必要（next が内部状態を変えるため）

    println!("{:?}", v1_iter.next()); // Some(1)
    println!("{:?}", v1_iter.next()); // Some(2)
    println!("{:?}", v1_iter.next()); // Some(3)
    println!("{:?}", v1_iter.next()); // None

    // ====================================================================
    // iter() / iter_mut() / into_iter() の違い
    // ====================================================================
    //
    // iter()      → &T（不変参照）を返す。元のコレクションはそのまま使える
    // iter_mut()  → &mut T（可変参照）を返す。要素を変更できる
    // into_iter() → T（所有権）を返す。元のコレクションは使えなくなる
    //
    // for ループは暗黙的に into_iter() を呼ぶ:
    //   for val in vec  → into_iter() — 所有権移動
    //   for val in &vec → iter()      — 不変参照
    //   for val in &mut vec → iter_mut() — 可変参照

    println!("\n==== iter() / iter_mut() / into_iter() ====");

    // iter(): 不変参照
    let v = vec![1, 2, 3];
    let total: i32 = v.iter().sum();
    println!("iter() sum = {total}");
    println!("v はまだ使える: {:?}", v);

    // iter_mut(): 要素を変更
    let mut v = vec![1, 2, 3];
    for val in v.iter_mut() {
        *val *= 2;
    }
    println!("iter_mut() で2倍: {:?}", v); // [2, 4, 6]

    // into_iter(): 所有権を消費
    let v = vec![1, 2, 3];
    let v2: Vec<i32> = v.into_iter().map(|x| x * 10).collect();
    println!("into_iter() + map: {:?}", v2); // [10, 20, 30]
    // println!("{:?}", v); // ← エラー! 所有権は into_iter() に移動済み

    // ====================================================================
    // 消費アダプタ（consuming adaptors）
    // ====================================================================
    //
    // next() を呼んでイテレータを「消費」するメソッド
    // 呼んだ後はイテレータは使えなくなる
    //
    // 例: sum(), collect(), count(), last(), nth(), for_each()

    println!("\n==== 消費アダプタ: sum() ====");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // v1_iter はもう使えない（sum が所有権を奪った）
    println!("合計: {total}"); // 6

    // ====================================================================
    // イテレータアダプタ（iterator adaptors）
    // ====================================================================
    //
    // イテレータを別のイテレータに変換するメソッド
    // 怠惰なので、消費するまで何も起きない
    //
    // 例: map(), filter(), zip(), enumerate(), take(), skip()

    println!("\n==== イテレータアダプタ: map + collect ====");
    let v1: Vec<i32> = vec![1, 2, 3];

    // map だけだと何も起きない（怠惰）
    // v1.iter().map(|x| x + 1); // ← 警告: unused `Map`

    // collect() で消費して初めて実行される
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("map(+1): {:?}", v2); // [2, 3, 4]

    // ====================================================================
    // filter でクロージャが環境をキャプチャする例
    // ====================================================================

    println!("\n==== filter で環境キャプチャ ====");
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    println!("サイズ10の靴: {:?}", in_my_size);
    // [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
}

// ====================================================================
// Shoe 構造体と shoes_in_size 関数
// ====================================================================
//
// filter はクロージャが true を返す要素だけを残す
// shoe_size は環境からキャプチャされる

#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter() で所有権を取得 → filter → collect
    // filter のクロージャが shoe_size をキャプチャ（不変借用）
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// ====================================================================
// テスト
// ====================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_next() {
        let v1 = vec![1, 2, 3];
        let mut iter = v1.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let total: i32 = v1.iter().sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }
}
