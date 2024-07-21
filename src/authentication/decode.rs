use super::{
    domain::{Header, Payload, Token, TOKEN_DELIMETER},
    error::DecodeError,
};
use base64::{
    engine::{general_purpose::URL_SAFE_NO_PAD, GeneralPurpose},
    Engine as _,
};

static DECODER: GeneralPurpose = URL_SAFE_NO_PAD;

pub fn decode(token: &str) -> Result<Token, DecodeError> {
    let segments = token.split(TOKEN_DELIMETER).collect::<Vec<&str>>();
    if segments.len() < 3 || segments.len() > 3 {
        return Err(DecodeError::InvalidTokenFormat);
    }

    let decoder = DECODER
        .decode(segments[0])
        .expect("Failed to decode payload string");
    let header_str = String::from_utf8(decoder).expect("Invalid UTF8 format");
    let header: Header =
        serde_json::from_str(&header_str).expect("Failed to parse to Payload struct");
    let decoder = DECODER
        .decode(segments[1])
        .expect("Failed to decode payload string");
    let payload_str = String::from_utf8(decoder).expect("Invalid UTF8 format");
    let payload: Payload =
        serde_json::from_str(&payload_str).expect("Failed to parse to Payload struct");
    let token = Token::new(header, payload);

    Ok(token)
}

#[cfg(test)]
mod test {
    use crate::authentication::{decode, domain::Payload, get_private_key_pk8, sign};

    #[test]
    fn should_have_err_with_invalid_token() {
        let test_cases = vec![
            ("sdfasf.sdfsd", "does not have full three parts"),
            ("sdfasf.sdfsd.sfaf.asfasf", "have more than three parts"),
        ];

        for (invalid_token, error_message) in test_cases {
            let decoded_token = decode(invalid_token);
            assert!(decoded_token.is_err(), "{}", error_message);
        }
    }

    #[test]
    fn should_decode_the_token() {
        let payload = Payload::new("Tom".into());
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        let token = sign(&payload, &secret);

        let decoded_token = decode(&token).expect("Failed to decode the token");

        assert_eq!(decoded_token.payload.name, payload.name);
    }
}
