fn main() {
    let number = 7;
    let condition = true;
    if number % 2 == 0{
        println!("{}", number);
    } else if  number % 7 == 0{
        println!("lucky");
    } else {
        println!("odd");
    }
    let vd = if condition { 5 } else { 1 };
    println!("vd = {}", vd);
    let mut counter = 5;
    loop {
        counter -= 1;
        println!("forever!");
        if counter < 0 {
            break;
        }
    }
    let vd1 = loop {
        counter += 1;
        if counter > 10 {
            break counter;
        }
    };
    println!("{}", vd1);
    println!("--------------");
    // while
    while counter > 0 {
        counter -= 1;
        println!("{}", counter);
    }
    for i in 1..4 {
        println!("{}", i);
    }
}
