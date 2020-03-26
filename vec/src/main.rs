use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 4];
    v.push(5);
    v.push(1);
    println!("{:?}", &v);
    v.push(5);
    v.push(1);
    v.push(5);
    v.push(1);
    v.remove(0);
    println!("{:?}", &v);
    let s = String::from("hello");
    let s1 = String::new();
    let s2 = "hello".to_string();
    println!("{}, {}", s, s2);
    let mut hm: HashMap<String, i32> = HashMap::new();
    hm.insert("blue".to_string(), 10);
    hm.insert("red".to_string(), 100);
    println!("{:?}", hm);
    println!("{:?}", hm.get(&"blue".to_string()));
}
