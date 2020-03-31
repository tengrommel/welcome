enum IpAddKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Change(i32, i32, i32),
}

impl Message {
    fn prin(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move{x, y}  => println!("Move x={}, y={}", x, y),
            Message::Change(a, b, c) => println!("Change a = {}, b = {}, c = {}", a, b, c),
            _ => println!("Write!")
            // Message::Write(&s) => println!("Write = {}", s)
        }
    }
}

fn main() {
    let i1 = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    let i2 = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
    let j1 = IpAddr2::V4(String::from("127.0.0.1"));
    let j2 = IpAddr2::V4(String::from("::1"));
    let l1 = IpAddr3::V4(1, 2, 3, 4);
    let l2 = IpAddr3::V6(String::from("ff"));
    let quit = Message::Quit;
    quit.prin();
}
