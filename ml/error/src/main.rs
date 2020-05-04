// Result<T, E>
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::io;
use std::io::Read;
use std::fs::File;
// 传播错误

fn main() {
    // let f = File::open("hello.txt");
    // let r = match f {
    //     Ok(file) => file,
    //     Err(e) => panic!(e)
    // };
    // panic!("crash")
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("{}", s),
        Err(e) => println!("err={:?}", e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };
    let mut s = String::new();// 读取成字符串
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// 问号
fn read_username_from_file_question() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();// 读取成字符串
    f.read_to_string(&mut s)?;
    Ok(s)
}