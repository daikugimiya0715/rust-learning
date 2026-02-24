// Step 3: ジェネリックなメソッド定義
// impl<T> で全型向けメソッド、impl Point<f32> で特定型のみのメソッドを書ける

struct Point<T> {
    x: T,
    y: T,
}

// すべての T に対するメソッド
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// f32 のみに対するメソッド
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 異なる型パラメータを持つ Point 同士を組み合わせる mixup の例
struct PointMixed<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMixed<X1, Y1> {
    // self の X1 と other の X2 を組み合わせて新しい Point を作る
    fn mixup<X2, Y2>(self, other: PointMixed<X2, Y2>) -> PointMixed<X1, Y2> {
        PointMixed {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    // 全型向けメソッド
    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());

    // f32 のみのメソッド
    let p_float = Point {
        x: 3.0_f32,
        y: 4.0_f32,
    };
    println!("原点からの距離: {}", p_float.distance_from_origin());

    // Point<i32> には distance_from_origin は呼べない
    // let p_int = Point { x: 3, y: 4 };
    // p_int.distance_from_origin(); // エラー！

    // mixup: 異なる型の Point を組み合わせ
    let p1 = PointMixed { x: 5, y: 10.4 };
    let p2 = PointMixed { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3: ({}, {})", p3.x, p3.y); // p3: (5, c)

    println!();
    println!("--- 注目ポイント ---");
    println!("impl<T> Point<T> → 全ての T に対してメソッドを定義");
    println!("impl Point<f32>  → f32 のみに対してメソッドを定義");
    println!("mixup のように、メソッド自体にも追加の型パラメータを持てる");
}
