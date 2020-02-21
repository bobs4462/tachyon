use super::request::HttpRequest;
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio::prelude::*;

// pub async fn not_found(mut socket: TcpStream) {
//     let content = match tokio::fs::read("/404.html").await {
//         Ok(content) => content,
//         Err(e) => {
//             panic!("Error reading file {}", e);
//         }
//     };
//     if let Err(e) = socket
//         .write_all(
//             &[
//                 "HTTP/1.1 404 Not Found\r\ncontent-type: text/html".as_bytes(),
//                 "; charset=UTF-8\r\n\r\n".as_bytes(),
//                 &content[..],
//             ]
//             .concat(),
//         )
//         .await
//     {
//         panic!("Error writing response: {}", e);
//     }
//     let _ = socket.shutdown(std::net::Shutdown::Write);
// }

pub async fn template_read<'a, 'b>(rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let content = match tokio::fs::read(
        "templates-db/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
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
    let path = match rqst.path {
        Some("") | Some("/") => "/index.html",
        Some(path) => path,
        None => panic!("PATH EMPTY"),
    };

    println!("{}", path);
    let content = match tokio::fs::read("gui".to_string() + path).await {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", e);
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
        "templates-db/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
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

pub async fn template_list<'a, 'b>(_rqst: HttpRequest<'a, 'b>, mut socket: TcpStream) {
    let mut entries = async_std::fs::read_dir("templates-db").await.unwrap();
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
