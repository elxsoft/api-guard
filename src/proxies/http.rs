use futures::FutureExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct RequestMetaInfo {
    method: String,
    uri: String,
    version: String,
    headers: HashMap<String, String>,
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    println!(" Starting Proxy HTTP Server ....");
    let server_key = "SERVER_ADDRESS";
    if env::var(server_key).is_err() {
        panic!(" Enviroment variable for server has not set yet ...");
    }

    let listen_addr = env::var(server_key)?.to_string();
    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let handle_connection = handle_connection(inbound).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(handle_connection);
    }

    Ok(())
}

async fn handle_connection(inbound: TcpStream) -> Result<(), Box<dyn Error>> {
    todo!()
}
