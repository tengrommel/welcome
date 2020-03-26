fn main() {
    // # Slices!
    let s = String::from("hello there");
    let back = first_world(&s);
    // s.clear();
    println!("{}", back);
    let a = [1,2,3];
    println!("{:?}", &a[..1]);
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s
}
