use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn parse(value: &str) -> Result<Name, String> {
        let regex =
            Regex::new(r"^([Є-Яа-яіїєёЁґҐ][Є-Яа-яіїєёЁґҐ'ʼ`-]{0,62}[Є-Яа-яіїєёЁґҐ])$").unwrap();
        if regex.is_match(value) {
            Ok(Name(value.to_string()))
        } else {
            Err("Name is not valid".to_string())
        }
    }
}
