#[derive(Debug)]
struct User {
    name: String,
    count: String,
    nonce: u64,
    active: bool,
}

fn main() {
    let xiaoming = User {
        name: String::from("xiaoming"),
        count: String::from("80001000"),
        nonce: 1000,
        active: true,
    };

    let mut xiaohuang = User {
        name: String::from("xiaoming"),
        count: String::from("80001000"),
        nonce: 1000,
        active: true,
    };
    xiaohuang.nonce = 20000;
    // 参数名字和字段名字同名的简写方法
    let name = String::from("xiaoxiao");
    let count = String::from("89077777");
    let nonce = 200000;
    let active = false;
    let user1 = User {
        name,
        count,
        nonce,
        active,
    };
    let user2 = User{
        name: String::from("teng"),
        ..user1
    };
    struct Point(i32, i32);
    let a = Point(10, 20);
    let b = Point(30, 11);
    println!("a.x = {}, b.y = {}", a.0, b.1);
    println!("{}",user2.nonce);
    println!("{}",user2.name );
    // 没有任何字段的结构体，特征
    struct A{};
    println!("小明： {:?}", user2);
    println!("小明： {:#?}", user2);
}
