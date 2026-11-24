use rsa::BoxedUint;

pub mod user;
pub mod client_messages;
pub mod server_messages;
pub mod wire_data;

pub trait Wire: Sized {
    fn read(payload: &mut &[u8]) -> Result<Self, String>;
    fn write(&self, out: &mut Vec<u8>);
}

pub trait NetworkSendable: Wire {
    const HEADER: u16;
}

pub fn encode<T: NetworkSendable>(msg: &T) -> Vec<u8> {
    let mut res = T::HEADER.to_le_bytes().to_vec();
    msg.write(&mut res);
    res
}

pub fn decode<T: NetworkSendable>(msg: &mut &[u8]) -> Result<T, String> {
    if u16::from_le_bytes((&msg[0..2]).try_into().unwrap()) != T::HEADER {
        return Err("incorrect message".to_string());
    }

    T::read(&mut &msg[2..])
}


