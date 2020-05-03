// enum Option<T> {
//     Some(T),
//     None
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1)
    }
}

// 使用方式
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("String"));
    let absent_number:Option<i32> = None;
    let x:i32 = 5;
    let y: Option<i32> = Some(5);

    // let sum = x + y;
    let mut temp = 0;
    match y {
        Some(i) => temp = i,
        None => println!("do nothing")
    }
    let sum = x + temp;
    println!("sum = {}", sum);
    let sum2 = x + y.unwrap();
    println!("sum2 = {}", sum2);
    let result = plus_one(y);
    println!("result: {}", result.unwrap());
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("{:?}", v);
    let v = vec![1,2,3];
    // for item in v {
    //     println!("{}", item)
    // }
    // {
    //     let v = vec![1, 2, 3];
    // }
    match v.get(1) {
        Some(x) => println!("{}", x),
        None => println!()
    }

    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    // 不可变的遍历
    for i in &v2 {
        println!("i = {}", i)
    }
    // 可变的遍历
    for i in &mut v2 {
        *i += 1
    }
    println!("{:?}", v2);
    // 使用枚举
    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };
    let v3 = vec![
        Context::Float(1.2),
        Context::Text(String::from("float")),
        Context::Int(-1),
    ];
    for i in &v3 {
        println!("item :{:?}", i);
    }
    let mut v = vec![1,2,3,4,5];
    let first = &v[0]; // mutable borrow occurs here 可变引用后使用不可变引用
    v.push(6);
    println!("first = {}", first);
}
