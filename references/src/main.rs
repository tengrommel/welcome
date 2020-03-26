fn main() {
    let s1 = String::from("ok1");
    // let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{} {}", r1, r2);  // if print error
    let r1 = &s1;
    let r2= &s1;
    println!("{} {}", r1, r2);// error
    let r = dangle();
}


fn dangle() -> &String{
    let s = String::from("ok");
    &s
}