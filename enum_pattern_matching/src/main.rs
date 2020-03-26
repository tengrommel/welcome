fn main() {
    // Create enums
    #[derive(Debug)]
    enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    // Accessing enums
    let c: Coin = Coin::Penny;
    // Enums with data
    #[derive(Debug)]
    enum Coin_Data{
        Penny,
        Nickel,
        Dime {x: i32, y: i32},
        Quarter(String)
    }

    // impl in enums
    impl Coin_Data{
        fn call(&self) {
            println!("{:?}", self);
        }
    }
    let c_d: Coin_Data = Coin_Data::Penny;
    c_d.call();
    // Rust doesn't have null
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let o: Option<i32> = Some(5);
    let o1: Option<String> = Some(String::from("hello"));

    match c {
        Coin::Penny => println!("{:?}", c),
        Coin::Nickel => println!("{:?}", c),
        Coin::Dime => println!("{:?}", c),
        Coin::Quarter => println!("{:?}", c)
    }
    // value binding in match
    fn value_in_cents(coin: Coin_Data) -> u8 {
        match coin {
            Coin_Data::Penny => 1,
            Coin_Data::Nickel => 2,
            Coin_Data::Dime => 3,
            Coin_Data::Quarter(some_string) => {
                println!("{:?}", some_string);
                23
            }
        }
    }
}
