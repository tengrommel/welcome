// 闭包
fn main() {
    let use_closure = || {
        println!("This is a closure")
    };
    use_closure();
    let add_one_v2 = |x: u32| -> u32 {
        x+1
    };
    let add_one_v3 = |x|{x+1};
    let add_one_v4 = |x| x+1;
    // 闭包定义会为每一个参数和返回值类型推导一个具体的类型，但是不能推导两次
    let b = add_one_v2(5);
    let c = add_one_v3(5);
    let d = add_one_v4(5);
    println!("b:{}, c: {}, d:{}", b, c, d);
    // 不能推导两次的例子
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    println!("s={}", s);
    // 不能正确地运行
    // let n = example_closure(5);
    // println!("s={}", n);
    // 捕捉环境变量
    let i = 1;
    let exe = |x| x+i;
    let r = exe(5);
    println!("r={}", r);
}

// 语法格式
fn add_one_v1(x: u32) -> u32 {
    x+1
}

