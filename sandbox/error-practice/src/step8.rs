// Step 8: unwrap / expect が適切な場面
// プロトタイプ・例・テストでは unwrap / expect でOK
// コンパイラより人間が多くの情報を持っている場合も unwrap して良い

use std::net::IpAddr;

pub fn run() {
    // ── 1. ハードコードされた値の parse ──
    // プログラマが「これは絶対に有効なIPアドレス」と分かっているので
    // unwrap / expect で問題ない
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("ハードコードされたIPアドレスは常に有効であるべき");
    println!("ホームアドレス: {}", home);

    print!("{}", home);

    let localhost: IpAddr = "::1".parse().expect("IPv6 ループバックも常に有効");
    println!("IPv6 ループバック: {}", localhost);

    // ── 2. expect に「なぜ成功するはずなのか」を書く ──
    // 悪い例（あまり情報がない）:
    //   .unwrap()
    //   .expect("parse failed")
    //
    // 良い例（なぜ成功するかの理由を書く）:
    //   .expect("ハードコードされたIPアドレスは常に有効であるべき")
    //
    // → パニックメッセージを見たとき、何が想定と違ったのかが分かる

    // ── 3. プロトタイプ段階では unwrap で素早く書く ──
    // 後でちゃんとエラー処理を入れるつもりなら、
    // まずは unwrap で動くコードを素早く書くのが合理的
    let numbers: Vec<i32> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("パースした数値: {:?}", numbers);

    println!("\n--- まとめ ---");
    println!("unwrap / expect が適切な場面:");
    println!("  1. プロトタイプ・例示・テストコード");
    println!("  2. 人間がコンパイラより多くの情報を持っているとき");
    println!("  3. expect には「なぜ成功するはずか」の理由を書く");
}
