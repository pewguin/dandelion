use crate::NetworkSendable;

pub struct TextMessage {
    message: String,
}

impl TextMessage {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl NetworkSendable for TextMessage {
    const HEADER: u16 = 0x0000;
    fn payload(&self) -> Vec<u8> {
        self.message.clone().into_bytes()
    }

    fn read_payload(payload: &[u8]) -> Result<Self, String> {
        let message = String::from_utf8(payload.to_owned()).map_err(|e| e.to_string())?;
        Ok(Self { message })
    }
}

#[cfg(test)]
mod tests {
    use crate::{decode, encode, peer_messages::TextMessage};

    #[test]
    fn text_message_reversible() {

        let s = "hello, this is a message for testing purposes!".to_string();
        let message = TextMessage::from(s.clone());

        assert!(decode::<TextMessage>(&encode(message)).unwrap().message == s);
    }
}
