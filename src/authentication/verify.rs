use super::domain::{Token, TokenError};

/// Verify the given token
///
/// # Examples
///
/// ```
/// use rush_booking::authentication::{Payload, Header, Token, TokenError, sign, verify};
/// let header = Header::new();
/// let payload = Payload::new("Tom".into());
/// let token = Token::new(header, payload);
///
/// assert!(verify(&token).is_ok());
/// ```
pub fn verify(token: &Token) -> Result<(), TokenError> {
    if token.is_expired() {
        return Err(TokenError::Expired);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use claims::assert_ok;

    use crate::authentication::{decode, domain::Payload, get_private_key_pk8, sign, verify};

    #[test]
    fn should_err_with_the_expired_token() {
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        // Define token has exp as time it created;
        let payload = Payload::new("Tom".into()).set_exp(Duration::from_secs(0));
        // Into future 1s
        thread::sleep(Duration::from_secs(1));
        let token = sign(&payload, &secret);

        let decoded_token = decode(&token).unwrap();

        assert!(verify(&decoded_token).is_err());
    }

    #[test]
    fn should_success_with_the_valid_token() {
        let secret =
            get_private_key_pk8("./private-key.pk8").expect("Failed to retrieve the private key");
        // Define token has exp in the future;
        let payload = Payload::new("Tom".into()).set_exp(Duration::from_secs(10));
        let token = sign(&payload, &secret);

        let decoded_token = decode(&token).unwrap();

        assert_ok!(verify(&decoded_token));
    }
}
