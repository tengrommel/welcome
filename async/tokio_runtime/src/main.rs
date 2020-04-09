use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        handle_client(socket).await?;
    }
}

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Received a connection from {}", remote_ip);
    let mut buf = [0; 1024];
    let n = socket.read(&mut buf).await?;
    let received = String::from_utf8(buf[0..n].to_vec())?;
    println!("They sent: {}", received);
        Ok(())
}