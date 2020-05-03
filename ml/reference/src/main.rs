fn main() {
    let mut s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;
    // let r3 = &mut s1; mutable borrow occurs here  有借用后不能使用引用
    println!("{} {}", r1, r2);
    // let ref_s = dangle();help: consider giving it a 'static lifetime: `&'static`
    // println!("s = {}", s);
    // dangle();

}

// 生命周期 对象已消失 悬垂引用
// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// }

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

