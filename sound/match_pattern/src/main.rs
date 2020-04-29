#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32,
}

#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Lounge,
}

fn main() {
    use self::Room::*;
    let t = Kitchen(4);
    println!("Hello from the {:?}", t);
    match t {
        Kitchen(n) => println!("The room is a kitchen with {} rooms", n),
        d => println!("{:?}", d)
    }
}
