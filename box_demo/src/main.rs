use crate::List::{Cons, Nil};

// box适用场景
// 1、当一个在编译时未知大小的类型，而又需要确切大小的上下文中适用这个类型的值的时候；
// 2、当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// 3、当希望拥有一个值只关心它的类型是否实现了特定trait而不是其具体类型时
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1,
                          Box::new(Cons(2,
                                        Box::new(Cons(3,
                                                      Box::new(Nil))))));
    let b = Box::new(5); // b存储于栈上 5存储在堆上 b指向5所在的内存
    println!("b = {}", b);
}
