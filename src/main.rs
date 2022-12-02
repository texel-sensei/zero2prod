use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("localhost:8000")?)?.await
}
