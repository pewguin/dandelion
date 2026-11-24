use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const FRAME_SIZE_MAX: u32 = 1 << 16;

pub async fn send_frame<W: AsyncWrite + Unpin>(stream: &mut W, payload: &[u8]) -> io::Result<()> {
    let len = payload.len();
    let mut buf = Vec::with_capacity(4 + len);
    buf.extend_from_slice(&(len as u32).to_le_bytes());
    buf.extend_from_slice(payload);
    stream.write_all(&buf).await?;
    stream.flush().await
}

pub async fn receive_frame<R: AsyncRead + Unpin>(stream: &mut R) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = u32::from_le_bytes(len_buf);

    if len > FRAME_SIZE_MAX {
        println!("len > frame size {} {}", len > FRAME_SIZE_MAX, FRAME_SIZE_MAX);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too big"));
    }

    let mut frame = vec![0u8; len as usize];
    stream.read_exact(&mut frame).await?;
    Ok(frame)
}
