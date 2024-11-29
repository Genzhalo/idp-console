use serde::{Deserialize, Serialize, Serializer};
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum SubmissionStatus {
    Received,
    Confirmed,
    Completed,
}

impl FromStr for SubmissionStatus {
    type Err = ();

    fn from_str(input: &str) -> Result<SubmissionStatus, Self::Err> {
        match input {
            "received" => Ok(SubmissionStatus::Received),
            "confirmed" => Ok(SubmissionStatus::Confirmed),
            "completed" => Ok(SubmissionStatus::Completed),
            _ => Err(()),
        }
    }
}

impl fmt::Display for SubmissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SubmissionStatus::Received => write!(f, "received"),
            SubmissionStatus::Confirmed => write!(f, "confirmed"),
            SubmissionStatus::Completed => write!(f, "completed"),
        }
    }
}

impl Serialize for SubmissionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
