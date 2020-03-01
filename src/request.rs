use httparse::{Header, Request, Status};
use tokio::net::TcpStream;
use tokio::prelude::*;
const BUF_SIZE: usize = 4096;

#[derive(Debug)]
pub struct HttpRequest<'headers, 'buf: 'headers> {
    /// The request method, such as `GET`.
    pub method: Option<&'buf str>,
    /// The request path, such as `/about-us`.
    pub path: Option<&'buf str>,
    /// The request headers.
    pub headers: &'headers mut [Header<'buf>],
    /// The request body.
    pub body: Option<&'buf str>,
    pub border: usize,
}

impl<'a, 'b: 'a> HttpRequest<'a, 'b> {
    pub fn new(
        buf: &'b Vec<u8>,
        headers: &'a mut [Header<'b>],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut req = Request::new(headers);
        let res = req.parse(&buf[..])?;
        let mut border: usize = 0;
        if let Status::Complete(stop) = res {
            border = stop;
        }
        let Request {
            method,
            path,
            version: _,
            headers,
        } = req;
        Ok(HttpRequest {
            method: method,
            path: path,
            headers: headers,
            body: None,
            border: border,
        })
    }

    pub async fn body_read(socket: &mut TcpStream, byte_to_read: usize) -> Vec<u8> {
        println!("here!!!");
        let mut buffer = [0_u8; 100000];
        println!(
            "BYTES 2nd ON SOCKET {:?}",
            socket.peek(&mut buffer).await.unwrap()
        );
        let mut buf = [0_u8; BUF_SIZE];
        let mut vec = Vec::with_capacity(byte_to_read);
        let mut _l = 0;
        let mut total_read = 0;
        loop {
            _l = socket.read(&mut buf[..]).await.unwrap();
            vec.append(&mut buf[.._l].to_vec());
            total_read += _l;
            if total_read >= byte_to_read - 2 {
                break;
            }
        }
        vec[..byte_to_read].to_vec()
    }

    pub async fn read(socket: &mut TcpStream) -> Vec<u8> {
        let mut buffer = [0_u8; 100000];
        println!(
            "BYTES 2nd ON SOCKET {:?}",
            socket.peek(&mut buffer).await.unwrap()
        );
        println!("BUFFER SIZE {}", socket.recv_buffer_size().unwrap());
        let mut buf = [0_u8; BUF_SIZE];
        let mut heap_buf: Vec<u8> = Vec::new();
        let mut l;
        loop {
            println!(
                "BYTES 2nd ON SOCKET {:?}",
                socket.peek(&mut buffer).await.unwrap()
            );
            l = socket.read(&mut buf[..]).await.unwrap();
            if l != BUF_SIZE && heap_buf.len() == 0 {
                break;
            } else if l != BUF_SIZE && heap_buf.len() != 0 {
                heap_buf.append(&mut buf[..l].to_vec());
                break;
            } else {
                heap_buf.append(&mut buf.to_vec());
            }
        }
        if heap_buf.len() > 0 {
            heap_buf
        } else {
            buf[..l].to_vec()
        }
    }
}
