use validator::Validate;

#[derive(Debug, Validate, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn parse(email: String) -> Result<Self, String> {
        let email = Email {
            value: email.to_string(),
        };
        match email.validate() {
            Ok(_) => Ok(email),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
