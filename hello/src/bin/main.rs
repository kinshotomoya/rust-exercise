extern crate hello;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));

        let get = b"GET / HTTP/1.1";
        let sleep = b"GET /sleep HTTP/1.1";
        if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(3))
        }

        let mut file = File::open("hello.html").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let response = format!("this is response: {}", content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
