#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Rc::new(Cons(Rc::new(RefCell::new(6)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(7)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
