use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("unexpected end of data")]
    UnexpectedEnd,
    #[error("bad line start: expected 0x0D, got 0x{0:02X}")]
    BadLineStart(u8),
    #[error("invalid extended token: prefix 0x{prefix:02X}, sub 0x{sub:02X}")]
    InvalidExtendedToken { prefix: u8, sub: u8 },
    #[error("invalid encoded line number")]
    InvalidLineNumber,
}
