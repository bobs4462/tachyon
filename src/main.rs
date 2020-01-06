use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("192.168.10.14:80").unwrap();

    for con in listener.incoming() {
        thread::spawn(|| connection_handle(con.unwrap()));
    }
}

fn connection_handle(mut stream: TcpStream) {
    let mut buffer = vec![0; 512];
    stream.read(&mut buffer).unwrap();
    // unsafe {
    //     println!("ReqUeST: {}", String::from_utf8_unchecked(buffer));
    // }
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") {
        stream
            .write(
                ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n".to_string()
                    + &std::fs::read_to_string("l.html").unwrap())
                    .as_bytes(),
            )
            .unwrap();
    } else if buffer.starts_with(b"GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        stream
            .write(
                ("HTTP/1.1 200 OK\r\n\r\n".to_string()
                    + &std::fs::read_to_string("l.html").unwrap())
                    .as_bytes(),
            )
            .unwrap();
    } else {
        stream
            .write(
                ("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string()
                    + &std::fs::read_to_string("404.html").unwrap())
                    .as_bytes(),
            )
            .unwrap();
    }
    // stream.flush().unwrap();
}
