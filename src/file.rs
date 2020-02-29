use super::error::{Error, ErrorKind};
use super::mime::MIMES;
use super::request::HttpRequest;
use super::response::Response;
use serde::{Deserialize, Serialize};
use tokio::prelude::*;

const DB: &str = "templates-db/index.json";

#[derive(Serialize, Deserialize)]
pub struct Template {
    name: String,
    file: String,
    comment: Option<String>,
    size: u32,
    data: serde_json::Value,
}

pub async fn template_read<'a, 'b>(
    rqst: HttpRequest<'a, 'b>,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let content = tokio::fs::read(
        "templates-db/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
    )
    .await?;
    let response = Response::new(b"200 OK".to_vec(), content, b"text/plain".to_vec());
    Ok(response)
}

pub async fn db_parse() -> Result<Vec<Template>, Box<dyn std::error::Error + Sync + Send>> {
    let content = std::fs::read(DB)?;
    let templates: Vec<Template> = serde_json::from_slice(&content)?;
    Ok(templates)
}

pub async fn file_read<'a, 'b>(
    rqst: HttpRequest<'a, 'b>,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let path = match rqst.path {
        Some("/") | Some("/engine") | Some("/syntax") | Some("/api-docs") | Some("/templates")
        | Some("/new") => "/index.html",
        Some(path) => path,
        None => panic!("IMPOSSIBLE ERROR: PATH EMPTY"),
    };

    let mime = MIMES
        .get(path.split('.').last().unwrap())
        .expect("IMPOSSIBLE ERROR: MIME NOT PRESENT IN DB");
    let content = tokio::fs::read("gui".to_string() + path).await?;
    let response = Response::new(b"200 OK".to_vec(), content, (*mime).as_bytes().to_vec());
    Ok(response)
}

pub async fn template_add<'a, 'b>(
    rqst: HttpRequest<'a, 'b>,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let mut f = tokio::fs::File::create(
        "templates-db/".to_string() + rqst.path.unwrap().split('/').last().unwrap(),
    )
    .await?;
    let body = match rqst.body {
        Some(body) => body.as_bytes(),
        None => {
            return Err(Box::new(Error::new(
                ErrorKind::EmptyBody("Пустое тело запроса".to_string()),
                None,
            )))
        }
    };

    f.write_all(body).await?;
    let response = Response::new(b"200 OK".to_vec(), b"OK".to_vec(), b"text/plain".to_vec());
    Ok(response)
}

pub async fn template_list<'a, 'b>(
    _rqst: HttpRequest<'a, 'b>,
) -> Result<Response, Box<dyn std::error::Error + Sync + Send>> {
    let db = match db_parse().await {
        Ok(templates) => templates,
        Err(e) => {
            return Err(Box::new(Error::new(
                ErrorKind::EmptyBody(format!("Ошибка {}", e)),
                None,
            )))
        }
    };
    let content = serde_json::to_vec(&db)?;
    let response = Response::new(b"200 OK".to_vec(), content, b"application/json".to_vec());
    Ok(response)
}
