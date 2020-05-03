#[derive(Debug)]
enum Message {
    Quit,
    Move{x:String, y:i32},
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn prin(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move {x, y} => println!("Move x = {}, y = {}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            Message::Write(s) => println!("Write = {}", s), //expected struct `std::string::String`, found reference
            // _ => println!("Quit")
        }
    }
}

fn main() {
    enum IpAddKind{
        V4,
        V6
    }
    struct IpAddr{
        kind: IpAddKind,
        address: String
    }
    let i1 = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    let i2 = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1")
    };
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));
    enum IpAddr3{
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));


    let quit = Message::Quit;
    println!("{:?}", quit);
    quit.prin();
    let write = Message::Write("fg".parse().unwrap());
    write.prin();
}
