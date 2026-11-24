use crate::{NetworkSendable, user::User};

struct ForwardingMessage {
    message: Vec<u8>,
    from: User,
}

