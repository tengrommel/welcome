use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // let v = vec![1,2,3];
    // v[99];
    // panic!("crash!!!")
    let f = match File::open("hello.txt") {
        Ok(file)=>file,
        Err(e) => panic!("{:?}", e),
    };
    let f = File::open("hello.txt").expect("off");
}

fn read_username() -> Result<String, io::Error> {
    let mut f = match File::open("username.txt") {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}