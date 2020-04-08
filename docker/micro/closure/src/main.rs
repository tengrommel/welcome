// 函数参数返回一个闭包
fn return_clo() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x|x+1)
}

fn wap_func<T>(t:T, v: i32) -> i32
    where T: Fn(i32) -> i32 {
    t(v)
}

fn func(v: i32) -> i32 {
    v + 1
}

fn main() {
    let a = wap_func(|x| x+1, 1);
    println!("a = {}", a);
    let b = wap_func(func, 1);
    println!("b = {}", b);
    let c = return_clo();
    println!("1+1={}", c(1));
}
