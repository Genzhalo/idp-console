use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Phone(String);

impl Phone {
    pub fn parse(value: &str) -> Result<Phone, String> {
        let regex = Regex::new(r"^(\+38)?\d{10}$").unwrap();
        if regex.is_match(value) {
            Ok(Phone(value.to_string()))
        } else {
            Err("Phone is not valid".to_string())
        }
    }
}
