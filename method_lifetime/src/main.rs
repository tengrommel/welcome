use std::fmt::Display;

#[derive(Debug)]
struct A<'b> {
    name: &'b str
}

fn get_a_str(s: &str) -> &str {
    s
}

struct StuA<'a> {
    name: &'a str
}

impl <'a> StuA<'a> {
    fn do_something(&self) -> i32 {
        3
    }
    fn do_something2(&self) -> &str{
        self.name
    }
}

fn function<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display{
    println!("ann is {}", ann);
    if x.len() < y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let n = String::from("hello");
    let a = A{name: &n};
    println!("a = {:#?}", a);
    let s = get_a_str(&n);
    println!("a = {}", s);
    let s = String::from("hello");
    let a = StuA{name: &s};
    println!("{}", a.do_something2());

    let s1 = String::from("i am s1");
    let s2 = String::from("i am s2");
    let ann = 199;
    let r = function(&s1, &s2, ann);
    println!("{}", r);
}
