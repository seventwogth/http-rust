use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::httpconn::build_http_get_request;
use crate::config::TEST_SERVER_ADDRESS;
use crate::config::EXAMPLE_SERVER_ADDRESS;

//TEST_CONNECTION (HTTPBIN)
pub async fn run_example_connection() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(EXAMPLE_SERVER_ADDRESS).await?;
    println!("CONNECTED TO ADDRESS: {}", EXAMPLE_SERVER_ADDRESS);

    let request = build_http_get_request("httpbin.org", "/");
    stream.write_all(request.as_bytes()).await?;
    println!("REQUEST SENT:\n{}", request);

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    println!("SERVER RESPONSE:\n{}", String::from_utf8_lossy(&buffer));

    Ok(())
}

pub async fn run_tcp_connection() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(TEST_SERVER_ADDRESS).await?;
    println!("CONNECTED TO ADDRESS: {}", TEST_SERVER_ADDRESS);

    let request = build_http_get_request("localhost", "/");
    stream.write_all(request.as_bytes()).await?;
    println!("REQUEST SENT:\n{}", request);

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    println!("SERVER RESPONSE:\n{}", String::from_utf8_lossy(&buffer));

    Ok(())
}
