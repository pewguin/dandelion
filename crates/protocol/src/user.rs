use rsa::{BoxedUint, PublicKey};

use crate::Wire;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub user_id: BoxedUint,
}

impl User {
    pub fn new(user_id: BoxedUint) -> Result<Self, String> {
        let new = Self { user_id };
        PublicKey::new(new.user_id.clone(), BoxedUint::from(0x10001_u64)).map_err(|e| e.to_string())?;
        Ok(new)
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::new(self.user_id.clone(), BoxedUint::from(0x10001_u64)).unwrap()
    }
}

// [length:4][user_id:length]
impl Wire for User {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let (len_bytes, rest) = payload.split_first_chunk::<4>().ok_or("short len")?;
        let length = u32::from_le_bytes(*len_bytes);
        *payload = rest;

        if payload.len() < length as usize { return Err("short payload".to_string()); }
        let (key_bytes, rest) = payload.split_at(length as usize);

        *payload = rest;

        let user_id = BoxedUint::from_le_slice(key_bytes, length * 8).map_err(|e| e.to_string())?;
        User::new(user_id)
    }

    fn write(&self, out: &mut Vec<u8>) {
        let bytes = self.user_id.to_le_bytes();
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use rsa::BoxedUint;

use crate::{Wire, user::User};

    #[test]
    fn user_survives_serialization() {
        let user_id = BoxedUint::from_be_slice(&[0xAB; 128], 128 * 8).unwrap();
        let user = User::new(user_id).unwrap();

        let mut bytes = Vec::new();
        user.write(&mut bytes);

        assert!(user == User::read(&mut bytes.as_slice()).unwrap());
    }
}
