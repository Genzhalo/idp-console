#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Region(String);

impl Region {
    pub fn parse(value: &str) -> Result<Region, String> {
        let values = vec![
            "АР Крим",
            "Вінницька",
            "Волинська",
            "Дніпропетровська",
            "Донецька",
            "Житомирська",
            "Закарпатська",
            "Запорізька",
            "ІваноФранківська",
            "Київська",
            "Кіровоградська",
            "Луганська",
            "Львівська",
            "Миколаївська",
            "Одеська",
            "Полтавська",
            "Рівненська",
            "Сумська",
            "Тернопільська",
            "Харківська",
            "Херсонська",
            "Хмельницька",
            "Черкаська",
            "Чернівецька",
            "Чернігівська",
        ];
        if values.contains(&value) {
            Ok(Region(value.to_string()))
        } else {
            Err("Region is not valid".to_string())
        }
    }
}
