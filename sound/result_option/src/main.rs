use std::collections::HashMap;
use std::env::args;

fn main() {
    let mut hm = HashMap::new();
    hm.insert(3, "Hello");
    hm.insert(5, "world");
    let r = hm.get(&9).unwrap_or(&"NoString");
    println!("{}", r);

    match "3".parse::<f32>(){
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }
}

fn getArg(n: usize) -> Result<String, String> {
    for (i, a) in args().enumerate() {
        if i == n {
            return Ok(a)
        }
    }
    Err("Not enought Args".to_string())
}