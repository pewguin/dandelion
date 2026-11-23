pub mod peer_messages;

pub trait NetworkSendable: Sized {
    const HEADER: u16;
    fn payload(&self) -> Vec<u8>;
    fn read_payload(payload: &[u8]) -> Result<Self, String>;
}

pub fn encode<T: NetworkSendable>(msg: T) -> Vec<u8> {
    let payload = msg.payload();
    let mut res = Vec::with_capacity(2 + payload.len());
    res.extend_from_slice(&T::HEADER.to_le_bytes());
    res.extend_from_slice(&payload);
    res
}

pub fn decode<T: NetworkSendable>(msg: &[u8]) -> Result<T, String> {
    if u16::from_le_bytes((&msg[0..2]).try_into().unwrap()) != T::HEADER {
        return Err("incorrect message".to_string());
    }

    T::read_payload(&msg[2..])
}


