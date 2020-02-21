use futures::stream::StreamExt;
use tachyon::router::route;
use tokio::net::TcpListener;

#[tokio::main(core_threads = 4)]
async fn main() {
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
