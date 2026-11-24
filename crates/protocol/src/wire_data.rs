use crate::{Wire, helpers::{read_bytes, read_length}};

// [str_len:4][ascii:str_len]
impl Wire for String {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let length = read_length(payload)?;

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

impl<W: Wire> Wire for Vec<W> {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        let length = read_length(payload)?;

        let mut res = Vec::with_capacity((length as usize).min(payload.len()));
        for _ in 0..length {
            res.push(W::read(payload)?);
        }

        Ok(res)
    }

    fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&(self.len() as u32).to_le_bytes());
        for item in self {
            item.write(out);
        }
    }
}

impl Wire for u8 {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        read_bytes::<1>(payload).map(|u| u[0])
    }

    fn write(&self, out: &mut Vec<u8>) {
        out.push(*self);
    }
}

impl Wire for u64 {
    fn read(payload: &mut &[u8]) -> Result<Self, String> {
        Ok(u64::from_le_bytes(read_bytes::<8>(payload)?))
    }

    fn write(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.to_le_bytes());
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

    #[test]
    fn string_vec_survives_serialization() {
        let strs: Vec<String> = vec!["apple", "pear", "mango", "strawberry", "lemon"].into_iter().map(|s| s.to_string()).collect();

        let mut bytes = Vec::new();
        strs.write(&mut bytes);

        assert!(strs == Vec::<String>::read(&mut bytes.as_slice()).unwrap());
    }

     #[test]
    fn u8_vec_survives_serialization() {
        let strs: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];

        let mut bytes = Vec::new();
        strs.write(&mut bytes);

        assert!(strs == Vec::<u8>::read(&mut bytes.as_slice()).unwrap());
    }
}
