use super::engine;
use super::file;
use super::request::HttpRequest;
use tokio::net::TcpStream;

pub async fn route(mut socket: TcpStream) {
    let buf: Vec<u8> = HttpRequest::read(&mut socket).await;
    let mut headers = [httparse::EMPTY_HEADER; 18];
    let rqst = HttpRequest::new(&buf, &mut headers).unwrap();
    let path = rqst.path.clone();

    if let None = path {
        return;
    }

    match path.unwrap().split('/').nth(1) {
        Some("render") => engine::render(rqst, socket).await,
        Some("reload") => engine::reload(rqst, socket).await,
        Some("raw") => file::template_read(rqst, socket).await,
        Some("add") => file::template_add(rqst, socket).await,
        Some("list") => file::template_list(rqst, socket).await,
        Some(_file) => file::file_read(rqst, socket).await,
        None => {}
    }
}
