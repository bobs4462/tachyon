use super::request::HttpRequest;
use async_std::sync::RwLock;
use std::process;
use tera::{Context, Tera};
use tokio::net::TcpStream;
use tokio::prelude::*;

lazy_static! {
    pub static ref TERA: RwLock<Tera> = RwLock::new(match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading template database {}", e);
            process::exit(1);
        }
    });
}

pub async fn reload<'a, 'b>(_rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let mut tera = TERA.write().await;
    tera.full_reload().unwrap();
    if let Err(e) = socket
        .write_all(
            &"HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\nOK".as_bytes(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
    let _ = socket.shutdown(std::net::Shutdown::Write);
}

pub async fn render<'a, 'b>(rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let tera = TERA.read().await;
    let body = match tera.render(
        rqst.path.unwrap().split('/').last().unwrap(),
        &Context::from_value(serde_json::from_str(rqst.body.unwrap()).unwrap()).unwrap(),
    ) {
        Ok(body) => body,
        Err(e) => panic!("Error rendering html: {}", e),
    };
    if let Err(e) = socket
        .write_all(
            ("HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n".to_string()
                + &body)
                .as_bytes(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
    let _ = socket.shutdown(std::net::Shutdown::Write);
}
