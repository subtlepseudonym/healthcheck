use std::convert::TryFrom;
use hyper::{Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url: String = match std::env::args().nth(1) {
        Some(url) => url,
        _ => std::process::exit(2),
    };

    let uri: Uri = match Uri::try_from(url) {
        Ok(u) => u,
        Err(_) => std::process::exit(3),
    };

    let client = Client::new();
    match client.get(uri).await {
        Ok(r) if r.status() == 200 => std::process::exit(0),
        _ => std::process::exit(4),
    }
}
