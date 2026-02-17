fn main() {
    let a = 10; // immutable object
    let b = a; // copy
    println!("{} {}", a, b); // borrow check!! - OK
    sample_intdayo();
    sample_intdayo();
    sample_intdayo();
    sample_string();
    ref_ref_ref();
    test1();
}

fn sample_intdayo() {
    let test = 10;
    let test_ref = &test; // reference
    println!("{} hello {}", test, test_ref)
}

fn sample_string() {
    let mut test = '1';
    let test_ref = &mut test;
    let a_mut_ref_move = test_ref;
    println!("{} hello ", a_mut_ref_move)
}

fn ref_ref_ref() {
    let a = String::from("hello world");
    let ref_ref_ref_a = &&&a;
    let ref_a = **ref_ref_ref_a;
    let b = ref_a;
    println!("a: {} ", a);
    println!("b: {} ", b);
}

fn test1() {
    let a = 0xff;
    let b = 98_222;
    println!("16進数：{}, 10進数：{}", a, b)
}
