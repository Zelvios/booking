use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            match handle_client(&mut stream).await  {
                Ok(()) => println!("Connection Successful"),
                Err(e) => eprintln!("Connection Error: {e}")
            }
        }).await?;
    }
}

async fn handle_client(stream: &mut TcpStream) -> Result<()> {
    let headers = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = include_str!("../index.html");
    let response = format!("{headers}{contents}");

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}