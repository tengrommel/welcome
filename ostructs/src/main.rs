// Create struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        &self.width + &self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        &self.width > &rect.width
    }
}

fn main() {
    // Instantiating Structs
    let mut rect = Rectangle{
        width: 10,
        height: 50
    };
    // Access Struct elements
    println!("Accessing element 'height': {}", rect.height);
    // Change Struct elements
    rect.height = 60;
    println!("Accessing element 'height': {}", rect.height);
    // Tuple Structs
    struct Color(u32, u32, u32);
    let rgb = Color(0, 0, 0);
    // Access smt in a t-struct
    println!("Accessing element t-struct:{}", rgb.0);
    println!("{:#?}", rect);
    let rect1 = Rectangle{width: 1, ..rect};
    println!("{:#?}", rect1);
    println!("area: {}", rect1.area());

}

fn new_rect(width: u32, height: u32) -> Rectangle{
    Rectangle{
        width,
        height,
    }
}