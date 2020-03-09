use futures::stream::StreamExt;
use std::io::Write;
use tachyon::router::route;
use tokio::net::TcpListener;

#[tokio::main(core_threads = 4, max_threads = 13)]
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
