fn main() {
    println!("Hello, world!");
    sample1();
    sample2();
}

fn sample1() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The Thrid element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("The third no third element."),
    }

    // let does_not_exit = &v[100]; // crashする
    let does_not_exit = v.get(100); // Optionが取れますね

    println!("does not exist {:?}", does_no2112t_exit);
}

fn sample2() {
    let mut v = vec![100, 32, 50];

    for i in &mut v {
        *i += 50;
    }

    println!("add collection: {:?}", v);
}
