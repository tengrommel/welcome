use std::fmt::Debug;

fn match_option<T: Debug>(o: Option<T>) {
    match o {
        Some(i) => println!("{:?}", i),
        None => println!("nothing")
    }
}

fn main() {
    let a = Some(3);
    let b = Some("hello");
    match_option(b);
    match_option(a);
}
