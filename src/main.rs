use futures::stream::StreamExt;
use std::io::Write;
use tachyon::router::route;
use tokio::net::TcpListener;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("logs/pid")
        .unwrap();
    f.write(format!("{}", std::process::id()).as_bytes())
        .expect("FAILED TO WRITE PID");
    drop(f);

    let mut listener = TcpListener::bind(tachyon::utils::address_get())
        .await
        .unwrap();
    let mut incoming = listener.incoming();
    let server = async move {
        while let Some(con) = incoming.next().await {
            match con {
                Ok(socket) => {
                    tokio::spawn(timeout(Duration::from_secs(10), route(socket)));
                }
                Err(err) => {
                    eprintln!("Error on getting connection = {:?}", err);
                }
            }
        }
    };
    server.await;
}
