use validator::validate_email;

#[derive(Debug)]
pub struct CustomerEmail(String);

impl CustomerEmail {
    pub fn parse(s: String) -> Result<CustomerEmail, String> {
        // TODO: add validation!
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email", s))
        }
    }
}

impl AsRef<str> for CustomerEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CustomerEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We just forward to the Display implementation of
        // the wrapped String.
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::CustomerEmail;
    use claims::assert_err;
    use fake::{faker::internet::en::SafeEmail, Fake};

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert_err!(CustomerEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(CustomerEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(CustomerEmail::parse(email));
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_is_accepted(valid_email: ValidEmailFixture) -> bool {
        // dbg!(valid_email.0);
        CustomerEmail::parse(valid_email.0).is_ok()
    }
}
