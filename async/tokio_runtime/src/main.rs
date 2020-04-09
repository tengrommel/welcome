use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use futures::{SinkExt, StreamExt};
use tokio_util::codec::{Framed, LinesCodec};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let binding = ":::10113";
    let mut listener = TcpListener::bind(&binding).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let _ = handle_client(socket).await;
        });
    }
}

async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let remote_ip = socket.peer_addr()?.ip();
    println!("Received a connection from {}", remote_ip);
    let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    // let mut client = Framed::new(socket, LinesCodec::new_with_max_length(1024));
    let query = match client.next().await {
        Some(Ok(q)) => q,
        _ => return Err("no query received".into()),
    };
    let (local_port, remote_port) = match parse_query(&query){
        Ok((l, r)) => (l, r),
        Err(e) => {
            let response = format!("{} : Error: INVALID-PORT\n", query);
            client.send(response).await?;
            return Err(e);
        }
    };
    println!("Local port: {}", local_port);
    println!("Remote port: {}", remote_port);

    Ok(())
}

fn parse_query(query: &str) -> Result<(u16, u16), Box<dyn Error + Send + Sync>> {
    let ports: Vec<&str> = query.split(",").map(|s| s.trim()).collect();
    Ok((ports[0].parse()?, ports[1].parse()?))
}