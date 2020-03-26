fn main() {
    let a = 34;
    let mut e: u8 = 0;
    let mut i = 0;
    let mut int:u8 = 255;
    let mut int: u8= 255;
    let mut float: f32 = 2.0;
    let mut b: bool = false;
    let mut c='够';

    println!("{}, {}, {}", a, b, c);

    // # Compound Types
    // ## Tuples
    let mut tup = (1, 2, 'c');
    println!("{:?}", tup);
    tup.2 = '都';
    // ## Arrays
    let mut arr = [1,2, 3];
    let mut arr1: [u8;3] = [1,2,3];
    println!("{:?}", arr1);
    arr1[0] = 20;
    println!("{:?}", arr1);
    // # Curiosities
    // ## Two's complement
}
