fn main() {
    // Functions
    let mut x = 1;
    println!("before function: {}", x);
    x = stuff(x);
    println!("After function: {}", x);
    // ## Statements
    // - ';'
    // - do not return a value.
    // ## Expressions
    // - no ';' have a 'return' keyword
}

fn stuff(x: u8) -> u8 {
    println!("Inside of function: {}", x);
    x+1
}