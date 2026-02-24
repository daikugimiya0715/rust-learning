#![allow(dead_code)]

mod step1;
mod step2;
mod step3;
mod step4;
mod step5;
mod step6;
mod step7;
mod step8;
mod step9;
mod step10;

fn main() {
    // 実行したいステップのコメントを外して cargo run
    // step1::run(); // Result の基本 — File::open と match
    // step2::run(); // ErrorKind で複数のエラーを区別
    // step3::run(); // unwrap と expect
    // step4::run(); // エラーの委譲 — match 版
    // step5::run(); // ? 演算子で簡潔に
    // step6::run().unwrap(); // ? を main で使う
    // step7::run(); // ? と Option
    // step8::run(); // unwrap / expect が適切な場面
    // step9::run(); // panic! vs Result のガイドライン
    step10::run(); // Guess 型 — 検証のための独自型
}
