fn main() {
    let str1 = "123456789".to_string();
    let str2 = "123".to_string();
    println!("The longest string is: {}", longest(&str1, &str2));
}

struct Game<'a> {
    level: &'a i32
}

impl<'a> Game<'a> {
    fn level<'b>(&'a self, x: &'b i32) -> &'a i32 {
        self.level
    }
}


fn longest<'a,'b>(x : &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn fist_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}