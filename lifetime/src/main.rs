// 1、Rust 中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样
// 2、生命周期的主要目标是避免悬垂引用
// 3、Rust编译器使用借用检查器来检查生命周期是否有效
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    let r = String::from("abc");
    r.as_str()
}

fn main() {
    let s1 = String::from("abcde");
    let s2 = String::from("ab");
    let r = longest(s1.as_str(), s2.as_str());
    println!("{}", r);
    let ss = get_str(s1.as_str(), s2.as_str());
    let s2 = a_str(s1.as_str(), s2.as_str());
}