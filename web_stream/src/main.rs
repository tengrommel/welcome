use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::{fs, time, thread};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "template/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "template/404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    let ten_millis = time::Duration::from_millis(10000);
    thread::sleep(ten_millis);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    for stream in listener.incoming() {
        // handle_client(stream?);
        let stream = stream.unwrap();
        let handle = thread::spawn(move||{
           handle_client(stream);
        });
        thread_vec.push(handle);
    }
    for handle in thread_vec {
        // handle.join().unwrap();
        handle.join().unwrap();
    }
    Ok(())
} // the stream is closed here