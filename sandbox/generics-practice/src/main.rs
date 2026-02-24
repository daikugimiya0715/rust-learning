#![allow(dead_code)]

mod step1;
mod step10;
mod step11;
mod step12;
mod step2;
mod step3;
mod step4;
mod step5;
mod step6;
mod step7;
mod step8;
mod step9;

fn main() {
    // 実行したいステップのコメントを外して cargo run
    // step1::run();  // ジェネリクスの動機 — コードの重複を排除する
    // step2::run();  // ジェネリックな構造体と列挙型
    // step3::run(); // ジェネリックなメソッド定義
    // step4::run();  // 単相化(monomorphization) — ランタイムコストゼロ
    // step5::run(); // トレイトの定義と実装
    // step6::run(); // デフォルト実装とオーバーライド
    // step7::run();  // トレイト境界 — Step 1 の問題を解決
    // step8::run();  // トレイトを返す / 条件付き実装
    // step9::run();  // derive で自動実装できるトレイトまとめ
    step10::run(); // ライフタイムの基本 — ダングリング参照の防止
    // step11::run(); // ライフタイム省略規則と構造体のライフタイム
    // step12::run(); // 'static と全要素の統合
}
