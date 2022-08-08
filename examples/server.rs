use hyper::server::conn::Http;
use hyper::{service::service_fn, Body, Request, Response};
use serde_derive::{Deserialize, Serialize};
use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

// const FILENAME: &str = "config.toml";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub server: DemoServer,
    pub dns: Dns,
    pub pinger: Pinger,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DemoServer {
    pub ip: String,
    pub port: i64,
    pub hostname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dns {
    pub ip: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pinger {
    #[serde(rename = "is_pinger")]
    pub is_pinger: bool,
}

async fn ponger(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("pong".into()))
}

// fn read_config_file() -> Root {
//     // Read the contents of the file using a `match` block
//     // to return the `data: Ok(c)` as a `String`
//     // or handle any `errors: Err(_)`.
//     let contents = match fs::read_to_string(FILENAME) {
//         // If successful return the files text as `contents`.
//         // `c` is a local variable.
//         Ok(c) => c,
//         // Handle the `error` case.
//         Err(_) => {
//             // Write `msg` to `stderr`.
//             panic!("Could not read file `{}`", FILENAME);
//             // Exit the program with exit code `1`.
//             // exit(1);
//         }
//     };
//     // Use a `match` block to return the
//     // file `contents` as a `Data struct: Ok(d)`
//     // or handle any `errors: Err(_)`.
//     match toml::from_str(&contents) {
//         Ok::<Root, _>(d) => d,
//         Err(_) => {
//             // Write `msg` to `stderr`.
//             panic!("Unable to load data from `{}`", FILENAME);
//         }
//     };
//     Root::default()
// }

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0u32));
    println!("Team Pongers");
    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let counter_clone = counter.clone();
        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(ponger))
                .await
            {
                println!("Error serving connection: {:?}", err);
            } else {
                let mut counter_clone = counter_clone.lock().unwrap();
                *counter_clone += 1;
                println!("Ponging {}", counter_clone);
            }
        });
    }
}
