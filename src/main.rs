use std::io::Result;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
#[tokio::main]
async fn main() -> Result<()> {
    let port = 8000;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buffer = [0; 128];
            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        println!("BYTES READ: {:?}", n);
                        stream
                            .write_all(&buffer[..n])
                            .await
                            .expect("Failed to write to stream");
                        println!("this is the buffer: {:?}", buffer);
                    },
                    Err(err) => {
                        println!("Failed to read from stream due to {:?}", err);
                    },
                }
            }
        });
    }
}
