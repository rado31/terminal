pub const RESET: &[u8; 7] = &[0x02, 0x03, 0x07, 0x30, 0x41, 0x6F, 0x5A];
pub const ENABLE: &[u8; 12] =
    &[0x02, 0x03, 0x0C, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xF7];
pub const DISABLE: &[u8; 12] =
    &[0x02, 0x03, 0x0C, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x0C];
pub const POLL: &[u8; 7] = &[0x02, 0x03, 0x07, 0x33, 0x41, 0x07, 0x70];
pub const ACK: &[u8; 6] = &[0x02, 0x03, 0x06, 0x00, 0xC2, 0x82];
pub const STACK: &[u8; 6] = &[0x02, 0x03, 0x06, 0x35, 0xEC, 0xE4];
pub const RETURN: &[u8; 6] = &[0x02, 0x03, 0x06, 0x36, 0x77, 0xD6];
pub const SECURITY: &[u8; 9] =
    &[0x02, 0x03, 0x09, 0x32, 0x00, 0x00, 0x00, 0x26, 0x1F];
