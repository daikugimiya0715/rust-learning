#[derive(Debug)]
enum List {
    // 次のリストを Rc と RefCell の両方で包む！
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 共有したいデータ「5」を RefCell と Rc で包む
    let value = Rc::new(RefCell::new(5));

    // リスト a を作る（先頭は value）
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // リスト b と c を作り、両方とも a を共有する（15.4と同じ）
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // ✨ここからが魔法！共有している value の中身を書き換える！
    // borrow_mut() を使って「5」に「+10」する
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
