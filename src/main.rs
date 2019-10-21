use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

mod server;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = server::ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("new connection");
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}
