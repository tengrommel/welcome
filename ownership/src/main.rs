fn main() {
    let x = 5;
    // s = s + ", how about you "; we can't do that
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);
}
