#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8
}

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    // print debug struct
    println!("{:?}", peter);
    // Instantiate a `Peter`
    let point: Point = Point{x: 10.3, y: 0.4};
    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    // Make a new point by using struct update syntax to use the fields other one
    let bottom_right = Point{x: 5.2, ..point};
    // `bottom_right.y` will be the same as `point.y` because we used that field from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // Destructure the point using a `let` binding
}
