
use crate::{client_messages::{AcknowledgeMessages, FetchMessages, TextMessage}, helpers::read_bytes, server_messages::{ForwardingMessage, MessageBatch}};

pub mod user;
pub mod client_messages;
pub mod server_messages;
pub mod wire_data;
pub mod frame;
mod helpers;

pub trait Wire: Sized {
    fn read(payload: &mut &[u8]) -> Result<Self, String>;
    fn write(&self, out: &mut Vec<u8>);
}

// HEADER FORMAT (bits):
// [origin:1][reserved:7][format:8]
// Origin is 1 if the message format is one from the server
// Reserved should be set to 0
// Format specifies what the particular format the message is in
pub trait NetworkSendable: Wire {
    const HEADER: u16;
}

pub fn encode<T: NetworkSendable>(msg: &T) -> Vec<u8> {
    let mut res = T::HEADER.to_le_bytes().to_vec();
    msg.write(&mut res);
    res
}

pub enum Message {
    Text(TextMessage),
    Fetch(FetchMessages),
    Acknowledge(AcknowledgeMessages),

    Forwarding(ForwardingMessage),
    Batch(MessageBatch),
}

pub fn decode(msg: &mut &[u8]) -> Result<Message, String> {
    let header = u16::from_le_bytes(read_bytes::<2>(msg)?);

    let decoded = match header {
        TextMessage::HEADER       => Message::Text(TextMessage::read(msg)?),
        FetchMessages::HEADER     => Message::Fetch(FetchMessages::read(msg)?),
        AcknowledgeMessages::HEADER => Message::Acknowledge(AcknowledgeMessages::read(msg)?),

        ForwardingMessage::HEADER => Message::Forwarding(ForwardingMessage::read(msg)?),
        MessageBatch::HEADER      => Message::Batch(MessageBatch::read(msg)?),
        _ => return Err(format!("invalid header {:#x}", header)),
    };

    if !msg.is_empty() {
        return Err(format!("{} trailing bytes after message", msg.len()));
    }
    Ok(decoded)
}

