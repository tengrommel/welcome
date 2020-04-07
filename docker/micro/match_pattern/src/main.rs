static HELLO_WORLD: &str = "Hello world";

// 静态变量和常量的区别：
// 1、静态变量有一个固定的内存地址（使用这个值总会访问相同的地址），常量则允许在任何被用到的时候复制
// 2、静态变量可以改变的
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