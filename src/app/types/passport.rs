use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Passport(String);

impl Passport {
    pub fn parse(value: &str) -> Result<Passport, String> {
        let regex = Regex::new(r"^[Є-ЯЁҐ]{2}\d{6}$|^\d{9}$").unwrap();
        if regex.is_match(value) {
            Ok(Passport(value.to_string()))
        } else {
            Err("Passport is not valid".to_string())
        }
    }
}
