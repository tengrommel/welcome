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
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after bind to b, a count = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after bind to c, a count = {}", Rc::strong_count(&a));
    }
    println!("count at end, a = {}", Rc::strong_count(&a));
}
