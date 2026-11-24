use crate::Wire;

// [str_len:4][ascii:str_len]
impl Wire for String {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let (len_bytes, rest) = payload.split_first_chunk::<4>().ok_or("short len")?;
        let length = u32::from_le_bytes(*len_bytes);
        *payload = rest;

        if payload.len() < length as usize { return Err("short payload".to_string()); }
        let (str_bytes, rest) = payload.split_at(length as usize);

        *payload = rest;

        String::from_utf8(str_bytes.to_vec()).map_err(|e| e.to_string())
    }

    fn write(&self, out: &mut Vec<u8>) {
        let bytes = self.as_bytes();
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&bytes);
    }
}

#[cfg(test)]
mod tests {
    use rsa::BoxedUint;

use crate::{Wire, user::User};

    #[test]
    fn string_survives_serialization() {
        let user_id = BoxedUint::from_be_slice(&[0xAB; 128], 128 * 8).unwrap();
        let user = User::new(user_id).unwrap();

        let mut bytes = Vec::new();
        user.write(&mut bytes);

        assert!(user == User::read(&mut bytes.as_slice()).unwrap());
    }
}
