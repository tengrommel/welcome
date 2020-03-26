fn main() {
    // s = s + ", how about you "; we can't do that
    let mut s = String::from("hello");
    let s2 = s;
    // s2 comes into scope s1 give ownership to s2
    // println!("{}", s); // error
    println!("{}", s2); // error
}
// double free error
// owner ship  s1 move to s2
