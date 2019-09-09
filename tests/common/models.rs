use domain_patterns::models::{ValueObject, AggregateRoot, Entity};
use regex::Regex;
use std::convert::TryFrom;
use uuid::Uuid;
use crate::common::{UserEvents, UserCreatedEvent};

#[derive(ValueSetup)]
pub struct Email {
    pub value: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ValidationError {
    #[fail(display = "Email failed to validate.")]
    EmailValidationError,
}


impl ValueObject<String> for Email {
    type ValueError = ValidationError;

    fn validate(value: &String) -> Result<(), ValidationError> {
        let email_rx = Regex::new(
            r"^(?i)[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
        ).unwrap();

        if !email_rx.is_match(value) {
            return Err(ValidationError::EmailValidationError);
        }

        Ok(())
    }

    fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(Entity, Clone)]
pub struct NaiveUser {
    id: Uuid,
    version: u64,
    first_name: String,
    last_name: String,
    email: Email,
}

impl AggregateRoot for NaiveUser {
    type Events = UserEvents;
}

impl NaiveUser {
    pub fn new(user_id: Uuid, first_name: String, last_name: String, email: String) -> Result<NaiveUser, ValidationError> {
        Ok(NaiveUser {
            id: user_id,
            version: 0,
            first_name,
            last_name,
            email: Email::try_from(email)?
        })
    }

    pub fn change_fname(&mut self, new_fname: String) {
        self.first_name = new_fname;
        self.version = self.next_version();
        let _created_event = UserCreatedEvent::new(self);
        // would publish event here - maybe create a mock bus for demonstration purposes.
    }
}

pub fn create_test_user(user_id: &Uuid) -> NaiveUser {
    // TODO: Update to return a Result type and pass error back.
    NaiveUser::new(
        user_id.clone(),
        "first_name".to_string(),
        "test_lname".to_string(),
        "test_email@email.com".to_string(),
    ).unwrap()
}
