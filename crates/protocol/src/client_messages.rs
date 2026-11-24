use rsa::{BoxedUint, PublicKey, traits::PublicKeyParts};

use crate::{NetworkSendable, Wire, user::User};

pub struct TextMessage {
    pub to: User,
    pub message: String,
}

impl TextMessage {
    pub fn new(message: String, to: &User) -> Self {
        Self { message, to: to.clone() }
    }
}

// [user_id_length:4][user_id:user_id_length][message_length:4][message:message_length]
impl Wire for TextMessage {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let user = User::read(payload)?;
        let message = String::read(payload)?;

        Ok(Self { to: user, message })
    }

    fn write(&self, out: &mut Vec<u8>) {
        self.to.write(out);
        self.message.write(out);
    }

}

impl NetworkSendable for TextMessage {
    const HEADER: u16 = 0x0000;
}

pub struct FetchMessages;

impl Wire for FetchMessages {
    fn read(_: &mut &[u8]) -> Result<Self, String> { Ok(Self) }
    fn write(&self, _: &mut Vec<u8>) {}
}

impl NetworkSendable for FetchMessages {
    const HEADER: u16 = 0x0001;
}

pub type AcknowledgeMessages = Vec<u64>;

impl NetworkSendable for AcknowledgeMessages {
    const HEADER: u16 = 0x0003;
}

#[cfg(test)]
mod tests {
    use rsa::BoxedUint;

    use crate::{Wire, client_messages::{AcknowledgeMessages, TextMessage, User}};

    #[test]
    fn text_message_survives_serialization() {
        let s = "hello, this is a message for testing purposes!".to_string();
        let user_id = BoxedUint::from_be_slice(&[0xFF; 128], 128 * 8).unwrap();
        let user = User::new(user_id).unwrap();
        let message = TextMessage::new(s.clone(), &user);

        let mut bytes = Vec::new();
        message.write(&mut bytes);

        let reformed: TextMessage = TextMessage::read(&mut bytes.as_slice()).unwrap();

        assert!(message.message == reformed.message);
        assert!(message.to == reformed.to);
    }

    #[test]
    fn message_acknowledgement_survives_serialization() {
        let ids = [1, 21, 312, 3, 21, 3, 124, 14, 1, 4, 80, 12, 35, 753, 0193, 312, 312134, 55674];
        let ack: AcknowledgeMessages = ids.to_vec();

        let mut bytes = Vec::new();
        ack.write(&mut bytes);

        assert!(ack == AcknowledgeMessages::read(&mut bytes.as_slice()).unwrap());
    }
}
