use std::net::UdpSocket;
use std::{io, str};

// 问题输入中文因３字节问题报错
fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8800")?;
    socket.connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send(input.as_bytes())?;

        let mut buffer = [0u8; 1500];
        socket.recv_from(&mut buffer)?;
        println!("recv: {}",
                 str::from_utf8(&buffer).expect("Could not write buffer as string"));
    }
}
