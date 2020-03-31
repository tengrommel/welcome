// fn main() {
//     let s1 = gives_ownership();
//     let mut s2 = String::from("hello");
//     let s3 = takes_and_gives_back(s2);
//     s2 = takes_and_gives_back(s3);
//     println!("s2 = {}", s2); // 值已经被使用
// }

// fn gives_ownership() -> String {
//     let s = String::from("hello");
//     s
// }

// fn takes_and_gives_back(s: String) -> String {
//     s
// }
// 引用：用法&，让我们创建一个指向值的引用，但是并不拥有它，
// 因为不拥有这个值，所以，当引用离开其值指向的作用域后也不会被丢弃
// 借用
fn calcute_length(s: &String) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(", world");
}

// // 野指针
// fn dangle() -> &String {
//     let s = String::from("staic");
//     &s
// }

fn main() {
    let s = String::from("hello");
    let h = &s[0..5];
    println!("h = {}", h);
    // let len = calcute_length(&s1);
    // println!("len = {}", len);
    // let ms = &mut s1;
    // modify_s(ms);
    // println!("s1 = {}", s1);
    // println!("len = {}", len);
    // let r1 = &s1;
    // let r2 = &s1;
    // let r3 = &mut s1;
    // println!("{}, {}, {}", r1, r2, r3);
    // let result = dangle();
}

// 可变引用之后不能使用之前的引用