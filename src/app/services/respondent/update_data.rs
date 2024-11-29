use std::borrow::Cow;

use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::app::types::{name::Name, passport::Passport, phone::Phone, region::Region};

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateData {
    #[validate(custom(function = "validate_first_name"))]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[validate(custom(function = "validate_last_name"))]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[validate(custom(function = "validate_passport"))]
    #[serde(rename = "passportId")]
    pub passport_id: Option<String>,
    #[validate(custom(function = "validate_idp_code"))]
    #[serde(rename = "IDPCode")]
    pub idp_code: Option<String>,
    #[validate(custom(function = "validate_phone"))]
    pub phone: Option<String>,
    #[validate(custom(function = "validate_region"))]
    pub region: Option<String>,
    pub children: Option<u8>,
}

fn validate_first_name(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(v) => match Name::parse(&v) {
            Ok(_) => Ok(()),
            Err(_) => Err(ValidationError::new("")
                .with_message(Cow::from("Прізвище містить заборонені символ"))),
        },
    }
}

fn validate_last_name(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(v) => {
            match Name::parse(&v) {
                Ok(_) => Ok(()),
                Err(_) => Err(ValidationError::new("")
                    .with_message(Cow::from("Ім'я містить заборонені символи"))),
            }
        }
    }
}

fn validate_phone(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(v) => match Phone::parse(&v) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(ValidationError::new("").with_message(Cow::from("Номер телефону неправильний")))
            }
        },
    }
}

fn validate_passport(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(v) => match Passport::parse(&v) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(ValidationError::new("").with_message(Cow::from("Паспортні дані неправильні")))
            }
        },
    }
}

fn validate_region(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(v) => match Region::parse(&v) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(ValidationError::new("").with_message(Cow::from("Область є обов'язковою")))
            }
        },
    }
}

fn validate_idp_code(value: &Option<String>) -> Result<(), ValidationError> {
    match value {
        Some(code) => {
            if code.len() > 32 {
                Err(ValidationError::new(
                    "Код ВПО не повинен перевищувати 32-ох символів",
                ))
            } else {
                Ok(())
            }
        }
        None => Ok(()),
    }
}
