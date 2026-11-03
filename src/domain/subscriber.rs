use unicode_segmentation::UnicodeSegmentation;
use validator::validate_email;

use crate::routes::subscriptions::FormData;

pub struct NewSubscriber {
    pub email: ValidEmail,
    pub name: SubscriberName,
}
impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = ValidEmail::parse(value.email)?;

        Ok(Self { email, name })
    }
}

#[derive(Debug)]
pub struct SubscriberName(String);
impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let has_forbidden_characters = s.chars().any(|ch| forbidden_characters.contains(&ch));

        if is_empty_or_whitespace || is_too_long || has_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ValidEmail(String);
impl ValidEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid email.", s))
        }
    }
}
impl AsRef<str> for ValidEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for ValidEmail {
    type Error = String;
    fn try_from(value: String) -> Result<Self, String> {
        Self::parse(value)
    }
}
impl std::fmt::Display for ValidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Calling the fmt trait method of the inner string
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber::{SubscriberName, ValidEmail};
    use claims::{assert_err, assert_ok};
    use fake::{Fake, faker::internet::en::SafeEmail};

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    // Name tests
    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }

    // Email tests
    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_valid(valid_email: ValidEmailFixture) -> bool {
        dbg!(&valid_email.0);
        ValidEmail::parse(valid_email.0).is_ok()
    }
    #[test]
    fn empty_email_is_rejected() {
        let email = "".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(ValidEmail::parse(email));
    }
}
