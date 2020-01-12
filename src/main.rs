use futures::stream::StreamExt;
// use http;
// #[macro_use]
extern crate serde_json;
// use serde_json::Value;
use async_std::sync::RwLock;
use std::process;
use tachyon::request::HttpRequest;
use tachyon::utils;
use tera::{Context, Tera};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
// use tokio::time::{delay_for, Duration};
#[macro_use]
extern crate lazy_static;

#[tokio::main(core_threads = 8)]
async fn main() {
    let mut listener = TcpListener::bind(utils::address_get()).await.unwrap();
    let mut incoming = listener.incoming();
    let server = async move {
        while let Some(con) = incoming.next().await {
            match con {
                Ok(mut socket) => {
                    let buf = HttpRequest::read(&mut socket).await;
                    tokio::spawn(route(buf, socket));
                }
                Err(err) => {
                    eprintln!("Error on getting connection = {:?}", err);
                }
            }
        }
    };
    server.await;
}
lazy_static! {
    pub static ref TERA: RwLock<Tera> = RwLock::new(match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading template database {}", e);
            process::exit(1);
        }
    });
}

pub async fn route(buf: Vec<u8>, socket: TcpStream) {
    let mut headers = [httparse::EMPTY_HEADER; 18];
    let rqst = HttpRequest::new(&buf, &mut headers).unwrap();
    let path = rqst.path.clone();
    match path.unwrap().split('/').nth(1) {
        Some("render") => render(rqst, socket).await,
        Some("raw") => template_read(rqst, socket).await,
        Some("add") => template_add(rqst, socket).await,
        Some("reload") => tera_reload(socket).await,
        Some(_action) => {}
        None => {}
    }
}

pub async fn template_read<'a, 'b>(rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let content = match tokio::fs::read(
        "templates/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
    )
    .await
    {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file {}", e);
        }
    };
    if let Err(e) = socket
        .write_all(
            &[
                "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=UTF-8\r\n\r\n".as_bytes(),
                &content[..],
            ]
            .concat(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
    println!("READ DONE! {}", String::from_utf8(content).unwrap());
    println!("WRITE DONE!");
    let _ = socket.shutdown(std::net::Shutdown::Write);
}

pub async fn template_add<'a, 'b>(rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let mut f = match tokio::fs::File::create(
        "templates/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
    )
    .await
    {
        Ok(file) => file,
        Err(e) => {
            panic!("Error reading file {}", e);
        }
    };
    if let Err(e) = f
        .write_all(&rqst.body.expect("file empty!").as_bytes())
        .await
    {
        panic!("Error writing response: {}", e);
    }
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

pub async fn tera_reload(mut socket: TcpStream) {
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
