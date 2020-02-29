use super::request::HttpRequest;
use super::response::Response;
use async_std::sync::RwLock;
use std::process;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TERA: RwLock<Tera> = RwLock::new(match Tera::new("templates-db/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error reading template database {}", e);
            process::exit(1);
        }
    });
}

pub async fn reload<'a, 'b>() -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let mut tera = TERA.write().await;
    tera.full_reload()?;
    let response = Response::new(b"200 OK".to_vec(), b"OK".to_vec(), b"text/plain".to_vec());
    Ok(response)
}

pub async fn render<'a, 'b>(
    rqst: HttpRequest<'a, 'b>,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let tera = TERA.read().await;
    let content = tera.render(
        rqst.path
            .expect("NOT PATH SPECIFIED")
            .split('/')
            .last()
            .expect("NO FILE NAME PROVIDED"),
        &Context::from_value(serde_json::from_str(rqst.body.expect("EMPTY BODY"))?)?,
    )?;
    let response = Response::new(
        b"200 OK".to_vec(),
        content.as_bytes().to_vec(),
        b"text/plain".to_vec(),
    );
    Ok(response)
}
