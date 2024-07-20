use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub const TOKEN_DELIMETER: &str = ".";
pub const DEFAULT_TOKEN_ALG: &str = "RS256";
pub const DEFAULT_TOKEN_TYPE: &str = "JWT";
pub const DEFAULT_TOKEN_TTL: u64 = 3600; // One hour

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
    pub name: String,
    pub exp: u64, // 2. Terminology - NumericDate
}

impl Payload {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exp: get_expired_unix_timestamp(DEFAULT_TOKEN_TTL),
        }
    }
}

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

pub struct Token {
    pub header: Header,
    pub payload: Payload,
}

impl Token {
    pub fn new(header: Header, payload: Payload) -> Self {
        Self { header, payload }
    }
}

fn get_expired_unix_timestamp(next: u64) -> u64 {
    let current: u64 = get_sys_time_in_secs();

    current + next
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn different_payloads_have_different_expired_time_based_on_order_of_creation() {
        let payload_one = Payload::new("Tom".into());
        thread::sleep(Duration::from_secs(1));
        let payload_two = Payload::new("Tom".into());

        assert!(payload_one.exp < payload_two.exp);
    }
}
