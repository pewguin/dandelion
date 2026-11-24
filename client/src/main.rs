use std::{io, };

use protocol::{client_messages::TextMessage, encode, frame::send_frame, user::User};
use rsa::BoxedUint;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:3000").await?;
    stream.set_nodelay(true)?;
    let user_id = BoxedUint::from_be_slice(&[0xAB; 128], 128 * 8).unwrap();
    let user = User::new(user_id).unwrap();

    for line in io::stdin().lines() {
        if let Ok(s) = line {
            let message = TextMessage::new(s, &user);
            send_frame(&mut stream, &encode(&message)).await.unwrap();
        }
    }

    Ok(())
}
