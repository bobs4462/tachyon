// use http;
// #[macro_use]
extern crate serde_json;
// use serde_json::Value;
use futures::stream::StreamExt;
use tachyon::router::route;
use tachyon::utils;
use tokio::net::TcpListener;
// use tokio::time::{delay_for, Duration};

#[tokio::main(core_threads = 4)]
async fn main() {
    let mut listener = TcpListener::bind(utils::address_get()).await.unwrap();
    let mut incoming = listener.incoming();
    let server = async move {
        while let Some(con) = incoming.next().await {
            match con {
                Ok(socket) => {
                    tokio::spawn(route(socket));
                }
                Err(err) => {
                    eprintln!("Error on getting connection = {:?}", err);
                }
            }
        }
    };
    server.await;
}
