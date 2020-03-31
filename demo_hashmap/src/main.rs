use std::collections::HashMap;
use std::option::Option::Some;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    let keys = vec![String::from("Blue"), String::from("Reb")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    let k = String::from("Blue");
    // let v = scores.get(&k);
    if let Some(v) = scores.get(&k) {
        println!("v = {}", v);
    }
    //遍历时会以任意顺序
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    // 直接插入值
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.entry(String::from("three")).or_insert(4);
    // 根据旧值更新一个值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    for (k, v) in map {
        println!("k={}, v={}", k, v);
    }
}
