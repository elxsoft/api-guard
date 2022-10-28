mod proxies;

use crate::proxies::*;
use std::env;

#[tokio::main]
async fn main() {
    let server_key = "SERVER_ADDRESS";
    if env::var(server_key).is_err() {
        panic!(" Enviroment variable for server has not set yet ...");
    }

    match env::var(server_key) {
        Ok(val) => println!("{server_key}: {val:?}"),
        Err(e) => println!("couldn't interpret {server_key}: {e}"),
    }

    match http::start().await {
        Ok(_result) => {
            println!(" Completed ...")
        }
        Err(err) => {
            eprintln!("Error: {:?}", err)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    #[test]
    fn test_if_server_address_exists() {
        let server_key = "SERVER_ADDRESS";
        match env::var(server_key) {
            Ok(val) => println!("{server_key}: {val:?}"),
            Err(e) => println!("couldn't interpret {server_key}: {e}"),
        }
    }
}
