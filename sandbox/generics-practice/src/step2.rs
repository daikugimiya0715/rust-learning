// Step 2: ジェネリックな構造体と列挙型
// 型パラメータを使って、さまざまな型で再利用できる構造体を作る

// 1 つの型パラメータ — x と y は同じ型でなければならない
struct Point<T> {
    x: T,
    y: T,
}

// 2 つの型パラメータ — x と y が異なる型でもOK
struct PointMixed<T, U> {
    x: T,
    y: U,
}

pub fn run() {
    // Point<T> — 同じ型
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("integer_point: ({}, {})", integer_point.x, integer_point.y);
    println!("float_point: ({}, {})", float_point.x, float_point.y);

    // これはコンパイルエラーになる（x: i32, y: f64 は Point<T> に合わない）
    // let wont_work = Point { x: 5, y: 4.0 };

    // PointMixed<T, U> — 異なる型もOK
    let mixed = PointMixed { x: 5, y: 4.0 };
    println!("mixed: ({}, {})", mixed.x, mixed.y);

    let both_integer = PointMixed { x: 5, y: 10 };
    println!("both_integer: ({}, {})", both_integer.x, both_integer.y);

    println!();
    println!("--- 標準ライブラリのジェネリック列挙型 ---");
    println!("Option<T>  = Some(T) | None");
    println!("Result<T, E> = Ok(T) | Err(E)");
    println!("どちらも同じジェネリクスの仕組みで作られている");

    // Option<T> と Result<T, E> の具体例
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;
    println!("some_number: {:?}, no_number: {:?}", some_number, no_number);

    let ok_result: Result<i32, String> = Ok(200);
    let err_result: Result<i32, String> = Err(String::from("エラー発生"));
    println!("ok_result: {:?}, err_result: {:?}", ok_result, err_result);
}
