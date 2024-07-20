use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ring::{rand::SystemRandom, rsa::KeyPair, signature};

use super::{
    domain::{Header, Options, Payload},
    error::MyError,
};

pub fn sign(payload: &Payload, secret: &[u8], options: Option<&Options>) -> String {
    let header = serde_json::to_string(&Header::new()).unwrap();
    let encoded_header = URL_SAFE_NO_PAD.encode(header);
    let payload = serde_json::to_string(payload).unwrap();
    let encoded_payload = URL_SAFE_NO_PAD.encode(payload);
    let signature = create_signature(secret, &encoded_header, &encoded_payload).unwrap();
    let encoded_signature = URL_SAFE_NO_PAD.encode(signature);

    format!("{encoded_header}.{encoded_payload}.{encoded_signature}")
}

fn create_signature(
    secret: &[u8],
    encoded_header: &str,
    encoded_payload: &str,
) -> Result<Vec<u8>, MyError> {
    let key_pair = KeyPair::from_pkcs8(secret).map_err(|e| MyError::BadPrivateKey)?;
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

pub fn get_private_key_pk8(path: &str) -> Result<Vec<u8>, MyError> {
    let private_key_path = std::path::Path::new(path);
    let private_key_pk8 = read_file(private_key_path)?;

    Ok(private_key_pk8)
}

fn read_file(path: &std::path::Path) -> Result<Vec<u8>, MyError> {
    use std::io::Read;

    let mut file = std::fs::File::open(path).map_err(|e| MyError::IO(e))?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .map_err(|e| MyError::IO(e))?;
    Ok(contents)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::authentication::domain::{Options, Payload, TOKEN_DELIMETER};
    use std::{thread, time::Duration};

    #[test]
    fn should_produce_different_signature_for_different_payloads() {
        let payload_one = Payload::new("Jerry".into());
        let payload_two = Payload::new("Tom".into());
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        let options = Options::new();
        let jwt_one = sign(&payload_one, &secret, Some(&options));
        let jwt_two = sign(&payload_two, &secret, Some(&options));

        let sign_one = jwt_one.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[2];
        let sign_two = jwt_two.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[2];

        assert_ne!(sign_one, sign_two);
    }

    #[test]
    fn should_produce_different_signature_for_same_payload_at_different_time() {
        let payload_one = Payload::new("Tom".into());
        // Wait for 1 second to create second payload
        thread::sleep(Duration::from_secs(1));
        let payload_two = Payload::new("Tom".into());
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        let options = Options::new();
        let jwt_one = sign(&payload_one, &secret, Some(&options));
        let jwt_two = sign(&payload_two, &secret, Some(&options));

        let sign_one = jwt_one.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[2];
        let sign_two = jwt_two.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[2];

        assert_ne!(sign_one, sign_two);
    }

    #[test]
    fn should_add_expiry_to_the_payload() {
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        let options = Options::new();
        let payload_one = Payload::new("Tom".into());
        let jwt_one = sign(&payload_one, &secret, Some(&options));

        let payload_one = jwt_one.split(TOKEN_DELIMETER).collect::<Vec<&str>>()[1];
        // Decode the payload and get "exp" key and its value is integer
        let decoder = URL_SAFE_NO_PAD
            .decode(payload_one)
            .expect("Failed to decode payload string");
        let payload_str = String::from_utf8(decoder).expect("Invalid UTF8 format");
        let payload: Payload =
            serde_json::from_str(&payload_str).expect("Failed to parse to Payload struct");

        assert!(payload.exp > 0);
    }
}
