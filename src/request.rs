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
}

impl<'a, 'b: 'a> HttpRequest<'a, 'b> {
    pub fn new(
        buf: &'b Vec<u8>,
        headers: &'a mut [Header<'b>],
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut req = Request::new(headers);
        let res = req.parse(&buf[..])?;
        let mut body = None;
        if let Status::Complete(stop) = res {
            if buf.len() > stop {
                body = Some(std::str::from_utf8(&buf[stop..buf.len()])?);
            }
        }
        // println!("{}", String::from_utf8(buf.clone()).unwrap());
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
            body: body,
        })
    }

    pub async fn read(socket: &mut TcpStream) -> Vec<u8> {
        let mut buf = [0_u8; BUF_SIZE];
        let mut heap_buf: Vec<u8> = Vec::new();
        let mut l;
        loop {
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
