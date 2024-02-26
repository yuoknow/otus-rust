#[derive(Debug)]
pub enum Response {
    Ok,
    Info(bool, f32),
    Unknown,
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] | [2, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Info(bytes[0] == 1, f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}
