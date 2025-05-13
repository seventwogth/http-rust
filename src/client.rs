use crate::config::EXAMPLE_SERVER_ADDRESS;
use crate::config::TEST_SERVER_ADDRESS;
use crate::httpconn::build_http_get_request;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn run_example_connection() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(EXAMPLE_SERVER_ADDRESS).await?;
    println!("CONNECTED TO ADDRESS: {}", EXAMPLE_SERVER_ADDRESS);

    let request = build_http_get_request("httpbin.org", "/");
    stream.write_all(request.as_bytes()).await?;
    println!("REQUEST SENT:\n{}", request);

    let mut buf = [0u8; 8 * 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let chunk = &buf[..n];
        print!("{}", String::from_utf8_lossy(chunk));
    }

    println!("\nSTREAM COMPLETE");
    Ok(())
}

pub async fn run_tcp_connection() -> std::io::Result<()> {
    let mut stream = TcpStream::connect(TEST_SERVER_ADDRESS).await?;
    println!("CONNECTED TO ADDRESS: {}", TEST_SERVER_ADDRESS);

    let request = build_http_get_request("localhost", "/");
    stream.write_all(request.as_bytes()).await?;
    println!("REQUEST SENT:\n{}", request);

    let mut buf = [0u8; 8 * 1024];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        let chunk = &buf[..n];
        print!("{}", String::from_utf8_lossy(chunk));
    }

    println!("\nSTREAM COMPLETE");
    Ok(())
}
