use crate::{NetworkSendable, Wire, helpers::read_length, user::User};

#[derive(Debug, PartialEq)]
pub struct ForwardingMessage {
    id: u64,
    from: User,
    message: Vec<u8>,
}

impl ForwardingMessage {
    pub fn new(id: u64, message: Vec<u8>, from: User) -> Self {
        Self { id, from, message }
    }
}

// [msg_id:8][user_id_length:4][user_id:user_id_length][message_length:4][message:message_length]
impl Wire for ForwardingMessage {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let id = u64::read(payload)?;
        let user = User::read(payload)?;
        let message = Vec::<u8>::read(payload)?;
        Ok(Self { id, from: user, message })
    }

    fn write(&self, out: &mut Vec<u8>) {
        self.id.write(out);
        self.from.write(out);
        self.message.write(out);
    }
}

impl NetworkSendable for ForwardingMessage {
    const HEADER: u16 = 0x8000;
}

pub type MessageBatch = Vec<ForwardingMessage>;

impl NetworkSendable for MessageBatch {
    const HEADER: u16 = 0x8001;
}

#[cfg(test)]
mod tests {
    use rsa::BoxedUint;
    use crate::{Wire, client_messages::TextMessage, server_messages::{ForwardingMessage, MessageBatch}, user::User};

    #[test]
    fn forwarding_message_survives_serialization() {
        let user_a_id = BoxedUint::from_be_slice(&[0xAB; 128], 128 * 8).unwrap();
        let user_a = User::new(user_a_id).unwrap();

        let user_b_id = BoxedUint::from_be_slice(&[0xCD; 128], 128 * 8).unwrap();
        let user_b = User::new(user_b_id).unwrap();

        let message = TextMessage::new("this is a test message for forwarding!".to_string(), &user_b);
        let mut msg_bytes = Vec::new();
        message.message.write(&mut msg_bytes);
        let forward = ForwardingMessage::new(1234, msg_bytes, user_a);
        
        let mut fwd_bytes = Vec::new();
        forward.write(&mut fwd_bytes);

        let recovered = ForwardingMessage::read(&mut fwd_bytes.as_slice()).unwrap();
        assert!(forward == recovered);
        assert!(message.message == String::read(&mut recovered.message.as_slice()).unwrap());
    }

    #[test]
    fn message_batch_survives_serialization() {
        let user_a_id = BoxedUint::from_be_slice(&[0xAB; 128], 128 * 8).unwrap();
        let user_a = User::new(user_a_id).unwrap();

        let user_b_id = BoxedUint::from_be_slice(&[0xCD; 128], 128 * 8).unwrap();
        let user_b = User::new(user_b_id).unwrap();

        let mut c = 0;

        let messages: Vec<ForwardingMessage> = vec!["apple", "pear", "mango", "strawberry", "lemon"]
            .into_iter()
            .map(|msg| {
                let txt = TextMessage::new(msg.to_string(), &user_b);
                let mut bytes = Vec::new();
                txt.write(&mut bytes);
                ForwardingMessage::new({let t = c; c += 1; t}, bytes, user_a.clone())
            })
            .collect();

        let batch: MessageBatch = messages;
        let mut bytes = Vec::new();
        batch.write(&mut bytes);
        
        assert!(batch == MessageBatch::read(&mut bytes.as_slice()).unwrap());
    }
}

