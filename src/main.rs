mod search;
use search::stackoverflow::{self, StackOverFlow};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let stack = stackoverflow::StackOverFlow {};
    let body = StackOverFlow::get_questions("single character string constant used as pattern").await;

    for (key, value) in body {
        println!("{}: {}", key, value);
    }
}
