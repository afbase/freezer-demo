use serde_derive::{Deserialize, Serialize};
use std::fs;
use tokio::{task, time};

const FILENAME: &str = "config.toml";

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

async fn pinger() {
    let config = read_config_file();
    let port_string = config.server.port.to_string();
    let port = port_string.as_str();

    let url_string = format!("http://{}:{}/ping", config.server.ip, port);
    println!("url_string: {}", url_string);
    let url = reqwest::Url::parse(url_string.as_str()).unwrap();
    println!("host:  {}", url_string);
    let connection_timeout_duration = core::time::Duration::from_secs(2);
    let client = reqwest::Client::builder()
        .connect_timeout(connection_timeout_duration)
        .build()
        .unwrap();
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let status_str = status.as_str();
            println!("Response: {}, {}", resp.text().await.unwrap(), status_str);
        }
        Err(e) => {
            println!("Response Error: {}", e);
        }
    }
}

fn read_config_file() -> Root {
    let mut result = Root::default();
    // Read the contents of the file using a `match` block
    // to return the `data: Ok(c)` as a `String`
    // or handle any `errors: Err(_)`.
    let contents = match fs::read_to_string(FILENAME) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            panic!("Could not read file `{}`", FILENAME);
            // Exit the program with exit code `1`.
            // exit(1);
        }
    };

    // Use a `match` block to return the
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    match toml::from_str(&contents) {
        Ok::<Root, _>(d) => {
            println!("toml object: {:?}", d);
            result = d;
        }
        Err(_) => {
            // Write `msg` to `stderr`.
            panic!("Unable to load data from `{}`", FILENAME);
        }
    };
    println!("result object: {:?}", result);
    result
}

#[tokio::main]
async fn main() {
    println!("7 eves pingers FTW!!!");
    let forever = task::spawn(async {
        let mut interval = time::interval(time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            pinger().await;
        }
    });
    forever.await;
}
