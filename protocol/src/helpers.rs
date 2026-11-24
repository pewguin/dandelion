pub(crate) fn read_bytes<const N: usize>(bytes: &mut &[u8]) -> Result<[u8; N], String> {
    let (n_bytes, rest) = bytes.split_first_chunk::<N>().ok_or("short len")?;
    *bytes = rest;
    Ok(*n_bytes)
}

pub(crate) fn read_length(bytes: &mut &[u8]) -> Result<u32, String> {
    Ok(u32::from_le_bytes(read_bytes::<4>(bytes)?))
}
