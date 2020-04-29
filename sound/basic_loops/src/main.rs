#[derive(Debug)]
pub struct User{
    name: String,
    age: i32,
    height: i32,
    shoesize: i32,
}

impl User {
    fn simple_string(&self) -> String {
        format!("{} - {} - {}cm - shoe:{}", self.name, self.age, self.height, self.shoesize)
    }
}


fn main() {
    loopto10();
    let u = User{
        name: String::from("Matt"),
        age: 33,
        height: 250,
        shoesize: 10,
    };
    println!("User is {:?}", u);
}

fn loopto10() {
    for n in 0..10 {
        println!("Hello {}", n);
    }
}

fn array_loop() {
    let v = vec![4, 6, 9, 10];
    for n in v {
        println!("{}", n);
    }
}