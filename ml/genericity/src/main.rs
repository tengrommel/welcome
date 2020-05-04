//　１、泛型是具体类型或者其它属性的抽象替代，用于减少代码重复
//　２、在函数定义中使用泛型
//　３、在结构体中使用泛型
//　４、枚举中的泛型
//　５、方法中的泛型
//　６、总结：使用泛型并不会造成程序性能上的损失。rust通过在编译时进行泛型代码的单态化来保证效率。单态化时通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// ---------------没有泛型-----------------

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

// for char
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// ---------------使用泛型--------------
// 泛型要求满足的条件 PartialOrd 可以比较 Copy 具有copy特征
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

// 在结构体中使用泛型
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl <T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
}

impl <T, U> Point2<T, U> {
    fn create_point<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2{
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    let p = Point{x:1.1, y:2.2};
    println!("x={}", p.get_x());
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest_i32(&number_list);
    println!("max_number = {}", max_number);
    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest_char(&char_list);
    println!("max_char = {}", max_char);
    let number_list = vec![1, 2, 23, 34, 8, 100];
    let max_number = largest(&number_list);
    println!("use generic max_number = {}", max_number);
    let char_list = vec!['a', 'y', 'b'];
    let max_char = largest(&char_list);
    println!("use generic max_char = {}", max_char);
    let inter = Point{x:1, y:2};
    println!("{:#?}", inter);
    let flo = Point{x:1.2, y:2.2};
    println!("{:#?}", flo);
    let flo_next = Point{x:"string", y:"ok"};
    println!("{:#?}", flo_next);
    let mul_generic = Point2{x:"rr", y:1};
    println!("{:#?}", mul_generic);
    let p1 = Point2{x: 5, y: 1.1};
    let p2 = Point2{x: "hello", y: 'c'};
    let p3 = p1.create_point(p2);
    println!("p3.x={}, p3.y={}", p3.x, p3.y);

}
