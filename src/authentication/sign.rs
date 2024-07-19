use std::u8;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ring::{rand::SystemRandom, rsa::KeyPair, signature};
use serde::{Deserialize, Serialize};

pub const DEFAULT_TOKEN_ALG: &str = "RS256";
pub const DEFAULT_TOKEN_TYPE: &str = "JWT";
pub const DEFAULT_TOKEN_TTL: u64 = 3600; // One hour

pub struct Options {
    expired_in: u64, // In seconds
}

impl Options {
    pub fn new() -> Self {
        Self {
            expired_in: DEFAULT_TOKEN_TTL,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Header {
    pub alg: String,
    pub typ: String,
}

impl Header {
    pub fn new() -> Self {
        Self {
            alg: DEFAULT_TOKEN_ALG.to_string(),
            typ: DEFAULT_TOKEN_TYPE.to_string(),
        }
    }
}

/// See [the RFC 7519] for more.
///
/// [the link]: https://www.rfc-editor.org/rfc/rfc7519
#[derive(Deserialize, Serialize)]
pub struct Payload {
    pub exp: u64, // 2. Terminology - NumericDate
}

impl Payload {
    pub fn new() -> Self {
        Self {
            exp: get_expired_unix_timestamp(DEFAULT_TOKEN_TTL),
        }
    }
}

pub fn sign(payload: &str, secret: &str, options: Option<&Options>) -> String {
    let header = serde_json::to_string(&Header::new()).unwrap();
    let encoded_header = URL_SAFE_NO_PAD.encode(header);
    let payload = serde_json::to_string(&Payload::new()).unwrap();
    let encoded_payload = URL_SAFE_NO_PAD.encode(payload);

    let signature = create_signature(secret.as_bytes(), &encoded_header, &encoded_payload).unwrap();

    let signature = String::from_utf8(signature).unwrap();

    format!("{encoded_header}.{encoded_payload}.{signature}")
}

fn get_expired_unix_timestamp(next: u64) -> u64 {
    let current: u64 = 1721400001;

    current + next
}

fn create_signature(
    secret: &[u8],
    encoded_header: &str,
    encoded_payload: &str,
) -> Result<Vec<u8>, MyError> {
    // Todo: Refactor
    let private_key_path = std::path::Path::new("./private-key.der");
    let private_key_der = read_file(private_key_path)?;
    let key_pair = KeyPair::from_der(&private_key_der).map_err(|_| MyError::BadPrivateKey)?;
    let message = format!("{encoded_header}.{encoded_payload}");
    let rng = SystemRandom::new();
    let mut signature = vec![0; key_pair.public().modulus_len()];
    key_pair
        .sign(
            &signature::RSA_PKCS1_SHA256,
            &rng,
            message.as_bytes(),
            &mut signature,
        )
        .map_err(|_| MyError::OOM)?;

    Ok(signature)
}

#[derive(Debug)]
enum MyError {
    IO(std::io::Error),
    BadPrivateKey,
    OOM,
    BadSignature,
}

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| MyError::IO(e))?;
    Ok(contents)
}
