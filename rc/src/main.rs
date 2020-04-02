use std::rc::Rc;
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
//
enum List {
    Cons(i32, Rc<List>),
    Nil
}
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    let b = Cons(3, a.clone());
    let c = Cons(4, Rc::clone(&a));
}
