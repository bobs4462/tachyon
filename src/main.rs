use futures::stream::StreamExt;
// use futures::Future;
// use http;
use httparse::{Request, Status};
use serde_json::{self, Value};
use std::process;
use tachyon::utils;
use tera::{Context, Tera};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
// use tokio::time::{delay_for, Duration};
#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind(utils::address_get()).await.unwrap();
    let mut incoming = listener.incoming();
    let server = async move {
        while let Some(con) = incoming.next().await {
            match con {
                Ok(socket) => {
                    println!("Got some connection from {:?}", socket.peer_addr());
                    let doc = Render::new("link.html".to_string(), Context::new());
                    tokio::spawn(doc.render(socket));
                }
                Err(err) => {
                    println!("Error on getting connection = {:?}", err);
                }
            }
        }
    };
    server.await;
}
lazy_static! {
    pub static ref TERA: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error obtaining address: {}", e);
            process::exit(1);
        }
    };
}

struct Render {
    template: String,
    context: Context,
}

impl Render {
    pub fn new(template: String, context: Context) -> Self {
        Render { template, context }
    }
    pub async fn render(self, mut socket: TcpStream) {
        let mut buf = [0_u8; 512];
        let _l = socket.read(&mut buf[..]).await;
        let mut headers = [httparse::EMPTY_HEADER; 18];
        let mut req = Request::new(&mut headers);
        let res = req.parse(&buf).unwrap();
        if let Status::Complete(stop) = res {
            println!("last char: {}", buf[stop]);
            if buf[stop] != 0 {
                let mut i = stop + 1;
                while buf[i] != 0 {
                    i += 1;
                }
                let b: Value = serde_json::from_slice(&buf[stop..i]).unwrap();
                println!("{:?}", b);
            }
        }

        // delay_for(Duration::from_secs(5)).await;
        println!(
            "{:?} {:?}, {}",
            req,
            res,
            String::from_utf8((&buf[..]).to_vec()).unwrap()
        );
        let body = match TERA.render(&self.template, &self.context) {
            Ok(body) => body,
            Err(e) => panic!("Error rendering html: {}", e),
        };
        if let Err(e) = socket
            .write_all(
                ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n".to_string()
                    + &body)
                    .as_bytes(),
            )
            .await
        {
            panic!("Error writing response: {}", e);
        }
        let _ = socket.shutdown(std::net::Shutdown::Write);
    }
}
