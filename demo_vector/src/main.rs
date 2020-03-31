fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    let v = vec![1, 2, 3];
    {
        let v1 = vec![1, 2, 3];
    }
    let one: &i32 = &v[0];
    println!("one = {}", one);
    println!("one = {}", *one);
    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("do nothing"),
    }
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    // 遍历
    // 不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }
    // 可变的遍历
    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }

    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };
    let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001),
    ];
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    // v.push(6);
    println!("first = {}", first);
}
