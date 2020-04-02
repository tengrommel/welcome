use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

use List::{Cons, Nil};
use std::option::Option::Some;

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("1, a rc count = {}", Rc::strong_count(&a));
    println!("1, a tail = {:?}", a.tail());
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("2, a rc count = {}", Rc::strong_count(&a));
    println!("2, b rc count = {}", Rc::strong_count(&b));
    println!("2, b tail = {:?}", a.tail());
    if let Some (link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("3, a rc count = {}", Rc::strong_count(&a));
    println!("3, b rc count = {}", Rc::strong_count(&b));
}
