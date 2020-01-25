use super::request::HttpRequest;
use async_std::sync::RwLock;
use futures::stream::StreamExt;
use futures::{future::Future, FutureExt};
use std::collections::HashMap;
use std::process;
use tera::{Context, Tera};
use tokio::net::TcpStream;
use tokio::prelude::*;

pub async fn route(mut socket: TcpStream) {
    let buf: Vec<u8> = HttpRequest::read(&mut socket).await;
    let mut headers = [httparse::EMPTY_HEADER; 18];
    let rqst = HttpRequest::new(&buf, &mut headers).unwrap();
    let path = rqst.path.clone();
    let map: HashMap<&'static str, fn(TcpStream) -> BoxFuture<()>> = {
        let mut map = HashMap::new();
        map.insert("not_found", not_found as fn(TcpStream) -> BoxFuture<()>);
        map
    };

    println!("path: {:?}", path);
    if let None = path {
        println!("{:?}", String::from_utf8(buf));
        index(socket).await;
        return;
    }

    match path.unwrap().split('/').nth(1) {
        Some("/") | Some("") => index(socket).await,
        Some("render") => render(rqst, socket).await,
        Some("raw") => template_read(rqst, socket).await,
        Some("add") => template_add(rqst, socket).await,
        Some("reload") => tera_reload(socket).await,
        Some("list") => read_dir(socket).await,
        Some(_file) => file_read(rqst, socket).await,
        None => {}
    }
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

type BoxFuture<T> = std::pin::Pin<std::boxed::Box<dyn Future<Output = T> + std::marker::Send>>;
pub fn not_found(mut socket: TcpStream) -> BoxFuture<()> {
    // -> std::pin::Pin<std::boxed::Box<dyn Future<Output = ()> + std::marker::Send>> {
    async move {
        let content = match tokio::fs::read("html/404.html").await {
            Ok(content) => content,
            Err(e) => {
                panic!("Error reading file {}", e);
            }
        };
        if let Err(e) = socket
            .write_all(
                &[
                    "HTTP/1.1 200 OK\r\ncontent-type: text/html".as_bytes(),
                    "; charset=UTF-8\r\n\r\n".as_bytes(),
                    &content[..],
                ]
                .concat(),
            )
            .await
        {
            panic!("Error writing response: {}", e);
        }
        let _ = socket.shutdown(std::net::Shutdown::Write);
    }
    .boxed()
}

pub async fn index(mut socket: TcpStream) {
    let content = match tokio::fs::read("html/index.html").await {
        Ok(content) => content,
        Err(e) => {
            panic!("Error reading file {}", e);
        }
    };
    if let Err(e) = socket
        .write_all(
            &[
                "HTTP/1.1 200 OK\r\ncontent-type: text/html".as_bytes(),
                "; charset=UTF-8\r\n\r\n".as_bytes(),
                &content[..],
            ]
            .concat(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
    let _ = socket.shutdown(std::net::Shutdown::Write);
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
    let _ = socket.shutdown(std::net::Shutdown::Write);
}

pub async fn file_read<'a, 'b>(rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let content = match tokio::fs::read("html/".to_string() + rqst.path.unwrap()).await {
        Ok(content) => content,
        Err(_) => {
            not_found(socket).await;
            return;
        }
    };
    let mime = match rqst.path.unwrap().split('.').last() {
        Some("js") => "text/javascript",
        Some("css") => "text/css",
        Some("ico") => "image/vnd.microsoft.icon",
        _ => "text/html",
    };
    if let Err(e) = socket
        .write_all(
            &[
                "HTTP/1.1 200 OK\r\ncontent-type: ".as_bytes(),
                mime.as_bytes(),
                "; charset=UTF-8\r\n\r\n".as_bytes(),
                &content[..],
            ]
            .concat(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
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

async fn read_dir(mut socket: TcpStream) {
    let mut entries = async_std::fs::read_dir(".").await.unwrap();
    let mut vec = Vec::new();
    while let Some(res) = entries.next().await {
        let entry = res.unwrap();
        vec.push(entry.file_name().to_string_lossy().to_string());
    }
    let vec = serde_json::to_vec(&vec).unwrap();

    if let Err(e) = socket
        .write_all(
            &[
                "HTTP/1.1 200 OK\r\ncontent-type: ".as_bytes(),
                "application/json".as_bytes(),
                "charset=UTF-8\r\n\r\n".as_bytes(),
                &vec.as_slice(),
            ]
            .concat(),
        )
        .await
    {
        panic!("Error writing response: {}", e);
    }
    let _ = socket.shutdown(std::net::Shutdown::Write);
}
