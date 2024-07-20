#[derive(Debug)]
pub enum MyError {
    IO(std::io::Error),
    BadPrivateKey,
    OOM,
    BadSignature,
}

#[derive(Debug)]
pub enum DecodeError {
    InvalidTokenFormat,
}
