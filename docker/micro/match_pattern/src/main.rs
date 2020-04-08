static HELLO_WORLD: &str = "Hello world";

// 静态变量和常量的区别：
// 1、静态变量有一个固定的内存地址（使用这个值总会访问相同的地址），常量则允许在任何被用到的时候复制
// 2、静态变量可以改变的
/*
1、在此节之前讨论过的都是安全的Rust，即Rust在编译时会强制执行的内存安全保证。
不会强制执行这类内存安全保证的，就是不安全的代码。
2、不安全的Rust存在两大原因：
（1）静态分析本质上是保守的，就意味着某些代码可能是合法的，但是Rust也会拒绝。在此情况下，可以使用不安全的代码。
（2）底层计算机硬件固有的不安全性。如果不允许进行不安全的操作，有些任务根本就完成不了。
3、unsafe 的特权
（1）解引用裸指针
（2）调用不安全的函数或者方法
（3）访问或修改可变静态变量
（4）实现不安全的trait
*/

fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("top = {}", top);
    } // 只要匹配Some(value)，就会一直运行
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }
    let p = (3, 5);
    print_point(&p);
    println!("{}", HELLO_WORLD);
}

fn print_point(&(x, y): &(i32, i32)) {
    println!("x:{}, y:{}", x, y);
}