use std::rc::Rc;
use List::{Cons, Nil};

// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }
// use List::{Cons, Nil};
enum List {
    Cons(i32, Rc<List>),
    Nil
}

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));// move发生后还在使用
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    eprintln!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    eprintln!("count after bind to b, a count = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        eprintln!("count after bind to b, a count = {}", Rc::strong_count(&a));
    }
    eprintln!("count after bind to b, a count = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));
    eprintln!("over")
}
