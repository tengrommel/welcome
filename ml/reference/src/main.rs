fn main() {
    let s1 = give_ownership();
    println!("{}", s1);
    let mut s2 = String::from("hello");
    let mut s3 = takes_and_gives_back(&s2);
    // s2 = takes_and_gives_back(s3);
    modify(&mut s2); // cannot borrow `s2` as mutable because it is also borrowed as immutable
    println!("s2 = {}", s2);
    println!("s2 = {}", s3);

    let mut ss = String::from("ok");
    modify(&mut ss);
    println!("ss = {}", ss);
}

fn give_ownership() -> String {
    let s = String::from("hello");
    s // 如果被使用将使用权交换给调用函数方
}

fn takes_and_gives_back(s: &String) -> &String {
    s
}

// 借用 借用后不能 再
fn modify(s: &mut String) {
    s.push_str(", world");
}

