//　１、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复
//　２、在函数定义中使用泛型
//　３、在结构体中使用泛型
//　４、枚举中的泛型
//　５、方法中的泛型
//　６、总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest_i32(&number_list);
    println!("max_number = {}", max_number);
    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest_char(&char_list);
}
