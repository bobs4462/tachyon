use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::env;

#[tokio::main]
async fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let mut listener = TcpListener::bind(&addr).await.unwrap();
    let mut incoming = listener.incoming();
    let server = async move {
        while let Some(con) = incoming.next().await {
            match con {
                Ok(mut socket) => {
                    println!("Got some connection from {:?}", socket.peer_addr());
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.split();
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => println!("wrote {} bytes", amt),
                            Err(err) => eprintln!("failed to echo, reason: {:?}", err),
                        }
                    });
                }
                Err(err) => {
                    println!("Error on getting connection = {:?}", err);
                }
            }
        }
    };
    server.await;
}
