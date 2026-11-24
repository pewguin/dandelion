use std::io;

use protocol::{Message, decode, frame::receive_frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:3000").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle(stream).await {
                println!("error on {} : {}", addr, e);
            }
        });
    }
}

async fn handle(mut stream: TcpStream) -> io::Result<()> {
    loop {
        let frame = match receive_frame(&mut stream).await {
            Ok(f) => f,
            Err(e) => {
                return match e.kind() {
                    io::ErrorKind::UnexpectedEof => Ok(()),
                    _ => Err(e)
                }
            }
        };

        let msg = decode(&mut frame.as_slice()).map_err(|s| io::Error::new(io::ErrorKind::Other, s))?;
        match msg {
            Message::Text(m) => println!("holy shit a text message: {}", m.message),
            _ => {}
        }
    }
}
