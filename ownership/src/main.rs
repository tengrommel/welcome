// 1 rust 通过所有权机制管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
// 2 堆和栈
// 编译的时候数据类型的大小是固定的，就是分配在栈上的
// 编译的时候数据类型大小不固定，就是分配堆上的
// 3 作用域
// 4 String内存回收
// 5 移动
// 6 clone
// 7 栈上数据拷贝
// 8 函数和作用域
// 常用的具有copy trait有
// 所有的整形
// 浮点型
// 布尔值
// 字符类型
// 元组

fn takes_ownership(something_string: String) -> String {
    println!("{}", something_string);
    something_string
}

fn make_copy(i: i32) {
    println!("i = {}", i);
}

fn main() {
    let x:i32 = 1;
    {
        let y: i32 = 1;
        println!("x = {}", x);
        println!("y = {}", y);
    }
    {
        let s1 = String::from("hello");
        // s1.push_str(" world");
        println!("s1 = {}", s1); // 在离开作用域时会调用drop方法
        let s2 = s1;
        println!("s2 = {}", s2);
        // println!("s1 = {}", s1);
        // clone
        let s3 = s2.clone();
        println!("s2 = {}", s2);
        println!("s3 = {}", s3);
    }
    // copy
    let a = 1;
    let b = a; // 在栈上 数据类型是就直接拷贝
    let s = "11";
    // let s = "11".to_string();
    let ss = s;
    println!("a = {} b = {}", a, b);
    println!("s = {} ss = {}", s, ss);
    let item = String::from("hello");
    takes_ownership(item);
    // println!("{}", item);
    let elem = 5;
    make_copy(elem);
    println!("{}", elem);
}
