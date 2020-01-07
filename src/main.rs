#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
// use futures::future::lazy;
#[macro_use]
extern crate serde_derive;
// use futures::Future;
// use hyper::client::HttpConnector;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::process;
use std::sync::{Arc, RwLock};
use tera::{Context, Tera};
use uuid::Uuid;

lazy_static! {
    pub static ref TERA: Tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error obtaining address: {}", e);
            process::exit(1);
        }
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = env::args();
    let addr = address_get(args);
    info!("Address is: {}", addr);
    let service = make_service_fn(|_: &AddrStream| {
        async move { Ok::<_, Infallible>(service_fn(move |req: Request<Body>| router(req))) }
    });
    let server = Server::bind(&addr).serve(service);
    if let Err(_) = server.await {
        process::exit(2);
    };
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index(),
        _ => four_oh_four(),
    }
}

static NOTFOUND: &[u8] = b"Oops! Not Found";

fn four_oh_four() -> Result<Response<Body>, Infallible> {
    let body = Body::from(NOTFOUND);
    Ok::<_, Infallible>(
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body)
            .unwrap(),
    )
}

fn index() -> Result<Response<Body>, Infallible> {
    let ctx = Context::new();
    let body = match TERA.render("link.html", &ctx) {
        Ok(body) => Body::from(body),
        Err(e) => panic!("Error rendering template: {}", e),
    };
    Ok::<_, Infallible>(
        Response::builder()
            .status(StatusCode::OK)
            .body(body)
            .unwrap(),
    )
}

fn address_get(mut args: env::Args) -> SocketAddr {
    let executable = args.next().unwrap();
    match args.next() {
        Some(addr) => match addr.parse() {
            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Error obtaining address: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!(
                "No arguments provided.\n\
                 Usage: {0} {{ip_address:port}}\n\
                 Example: {0} 127.0.0.1:8080",
                executable
            );
            process::exit(1);
        }
    }
}
