use validator::Validate;

#[derive(Debug, Validate, Clone, PartialEq)]
pub struct Password {
    #[validate(length(min = 8))]
    pub value: String,
}

impl Password {
    pub fn parse(password: &str) -> Result<Self, String> {
        let password = Password {
            value: password.to_string(),
        };
        match password.validate() {
            Ok(_) => Ok(password),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.value
    }
}
