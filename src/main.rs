use std::io::Result;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
#[tokio::main]
async fn main() -> Result<()> {
    let port = 8000;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer: Vec<u8> = Vec::new();
            loop {
                let mut chunk = [0; 128];
                match stream.read(&mut chunk).await {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("BYTES READ: {:?}", n);
                        buffer.extend_from_slice(&chunk[..n]);
                        if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
                            let body = "hello world";
                            let response = format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Length: {}\r\n\
                                Content-Type: text/plain\r\n\
                                Connection: close\r\n\
                                \r\n\
                                {}",
                                body.len(),
                                body
                            );
                            let _ = stream.write_all(response.as_bytes()).await;
                        }
                    }
                    Err(err) => {
                        println!("Failed to read from stream due to {:?}", err);
                    }
                }
            }
        });
    }
}
