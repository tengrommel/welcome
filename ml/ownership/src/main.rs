//　１、rust通过所有权机制来管理内存，编译器在编译就会根据所有权规则对内存的使用进行检查
//　２、堆和栈
// 编译的时候数据的类型大小是固定的，就是分配在栈上的
// 编译的时候数据类型大小不固定，就是分配在堆上
//　３、作用域
//　４、String内存回收
//　５、移动 move
//　６、clone
//　７、栈上数据拷贝
//　８、函数的作用域

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
}

fn make_copy(i:i32) {
    println!("i = {}", i);
}

fn main() {

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); borrow of moved value: `s`
    let number = 1;
    make_copy(1);
    println!("{}", number);



    let x:i32 = 1;
    {
        let y:i32 = 1;
        println!("{}", x);
        println!("y = {}", y);
    }
    {
        let s1 = String::from("hello");// 定义在堆上的
        // s1.push_str(" world");
        println!("s1 = {}", s1);
        // String类型离开作用域的时候会调用drop方法
        let s2 = s1; // s1无效　相当于move
        println!("s2 = {}", s2);
        // println!("s1 = {}", s1); // value borrowed here after move
        let s3 = s2.clone(); // 类似深拷贝
        println!("s3 = {}", s3);
        println!("s2 = {}", s2);
    }

    // 栈上可以使用
    // copy trait 浮点型　布尔值　字符串　整形　元组
    let a = 1;
    let b = a;
    println!("a = {} b = {}", a, b);
    println!("Hello, world!");
}
