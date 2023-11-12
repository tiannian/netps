#[derive(Debug)]
pub struct PasswordHash(pub [u8; 56]);

impl Default for PasswordHash {
    fn default() -> Self {
        Self([0u8; 56])
    }
}

pub const CRLF: u16 = 0x0d0a;

pub const CMD_CONNECT: u8 = 0x01;
pub const CMD_UDP: u8 = 0x03;

pub const ATYPE_V4: u8 = 0x01;
pub const ATYPE_DOMAIN_NAME: u8 = 0x03;
pub const ATYPE_V6: u8 = 0x04;
