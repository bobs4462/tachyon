use super::engine;
use super::file;
use super::request::{self, HttpRequest};
use super::response::Response;
use tokio::net::TcpStream;
use tokio::prelude::*;
// use tokio::time::{timeout, Duration};

pub async fn route(mut socket: TcpStream) {
    let buf: Vec<u8> = request::read(&mut socket).await;
    let mut headers = [httparse::EMPTY_HEADER; 18];
    let mut rqst = HttpRequest::new(&buf, &mut headers).expect("COULDN'T CREATE REQUEST");
    let mut body = Vec::new();
    if rqst.method.unwrap() == "POST" || rqst.method.unwrap() == "PUT" {
        if let Some(header) = rqst
            .headers
            .iter()
            .find(|i| i.name.to_lowercase() == "content-length")
        {
            if rqst.border != 0 {
                body.extend_from_slice(&buf[rqst.border..]);
            }
            let cl = String::from_utf8(header.value.to_vec())
                .unwrap()
                .parse::<usize>()
                .unwrap();
            loop {
                if cl > body.len() {
                    let data = request::read(&mut socket).await;
                    if data.len() == 0 {
                        panic!("CONNECTION TIMEOUT");
                    }
                    body.extend(&data);
                } else {
                    break;
                }
            }
            rqst.body = Some(std::str::from_utf8(&body[..]).unwrap());
        }
    }
    let path = rqst.path.clone();

    if let None = path {
        return;
    }

    let result = match path.expect("PATH EMPTY").split('/').nth(1) {
        Some("render") => engine::render(rqst).await,
        Some("reload") => engine::reload().await,
        Some("raw") => file::template_read(rqst).await,
        Some("add") => file::template_add(rqst).await,
        Some("list") => file::template_list(rqst).await,
        Some(_file) => file::file_read(rqst).await,
        _ => panic!("IMPOSSIBLE ERROR: PATH NOT PRESENT"),
    };

    let response = match result {
        Ok(ok_response) => ok_response.compose(),
        Err(e) => build_error_response(e),
    };

    socket
        .write_all(&response)
        .await
        .expect("FATAL ERROR: FAILED TO WRITE TO STREAM");
    socket
        .shutdown(std::net::Shutdown::Both)
        .expect("FATAL ERROR: FAILED TO CLOSE THE STREAM");
}

fn build_error_response(e: Box<dyn std::error::Error>) -> Vec<u8> {
    let response = Response::new(
        b"500 Internal server error".to_vec(),
        "Ошибка, обратитесь к системному администратору"
            .to_string()
            .into_bytes(),
        b"text/plain".to_vec(),
    );
    eprintln!("{}", e);
    response.compose()
}
