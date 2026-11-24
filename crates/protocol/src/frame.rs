use std::{io::{self, Read, Write}, net::TcpStream};

const FRAME_SIZE_MAX: u32 = 2^8;

pub fn send_frame(stream: &mut TcpStream, header: u16, payload: &[u8]) -> io::Result<()> {
    let len = 2 + payload.len() as u32;
    let mut buf = Vec::with_capacity(4 + len as usize);
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&header.to_le_bytes());
    buf.extend_from_slice(payload);
    stream.write_all(&buf)
}

pub fn receive_frame(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf)?;
    let len = u32::from_le_bytes(len_buf);

    if len > FRAME_SIZE_MAX {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "frame too big"));
    }

    let mut frame = vec![0u8; len as usize];
    stream.read_exact(&mut frame)?;
    Ok(frame)
}
