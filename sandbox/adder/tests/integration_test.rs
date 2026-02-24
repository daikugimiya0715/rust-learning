// ====================================================================
// 11.3: 結合テスト (integration tests)
// ====================================================================
//
// tests/ ディレクトリのファイルは独立したクレートとしてコンパイルされる
// → pub な API のみ使える（外部利用者と同じ視点でテスト）
// → use ライブラリ名 でインポートする

use adder;

mod common;

#[test]
fn it_adds_two() {
    // 共通セットアップを呼ぶ
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn it_adds() {
    assert_eq!(10, adder::add(3, 7));
}

#[test]
fn greeting_works() {
    let result = adder::greeting("World");
    assert!(result.contains("World"));
}

#[test]
fn rectangle_can_hold() {
    let larger = adder::Rectangle {
        width: 10,
        height: 10,
    };
    let smaller = adder::Rectangle {
        width: 5,
        height: 5,
    };
    assert!(larger.can_hold(&smaller));
}
