mod client;
mod httpconn;
mod config;

#[tokio::main]
async fn main() {
    if let Err(e) = client::run_example_connection().await {
        eprintln!("Error occured: {}", e);
    }

    println!("\n\n---!!!ACTUAL CONNECTION!!!---\n\n");

    if let Err(e) = client::run_tcp_connection().await {
        eprintln!("Error occured: {}", e);
    }
}
