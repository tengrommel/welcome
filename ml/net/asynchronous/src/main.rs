use futures::join;
use futures::executor;
use futures::future::join3;
use std::net::TcpStream;
use std::io::{Write, BufReader, BufRead};

fn use_server(server: &str, port: u16, content: &str) -> std::io::Result<()> {
    let mut stream = TcpStream::connect((server, port))?;
    let _ = stream.write(content.as_bytes())?;

    let mut reader = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer)?;
    println!("recv from server: {}", str::from_utf8(&buffer).unwrap());
    Ok(())
}

async fn async_use_server(server: &str, port: u16, content: &str) {
    use_server(server, port, content).unwrap();
}

async fn use_all_server() {
    let f1 = async_use_server("127.0.0.1", 8080, "use server1 127.0.0.1:8080");
    let f2 = async_use_server("127.0.0.1", 8081, "use server1 127.0.0.1:8081");
    join!(f1, f2);
}

fn main() -> std::io::Result<()>{
    let f = use_all_server();
    executor::block_on(f);
    Ok(())
}
