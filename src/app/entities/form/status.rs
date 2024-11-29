use serde::{Deserialize, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum FormStatus {
    Draft,
    Open,
    Close,
}

impl FromStr for FormStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<FormStatus, Self::Err> {
        match input {
            "open" => Ok(FormStatus::Open),
            "close" => Ok(FormStatus::Close),
            "draft" => Ok(FormStatus::Draft),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FormStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormStatus::Open => write!(f, "open"),
            FormStatus::Draft => write!(f, "draft"),
            FormStatus::Close => write!(f, "close"),
        }
    }
}

impl Serialize for FormStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
