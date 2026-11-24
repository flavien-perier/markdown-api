use jiff::civil::DateTime;
use serde::{self, Deserialize, Deserializer, Serializer};

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M";
const DATE_TIME_SECONDS_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

/// Dates are written without a time zone and are interpreted as UTC.
pub fn serialize<S>(date: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.strftime(DATE_TIME_SECONDS_FORMAT).to_string();
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let formats = [DATE_TIME_SECONDS_FORMAT, DATE_TIME_FORMAT];

    for format in formats.iter() {
        if let Ok(dt) = DateTime::strptime(format, &s) {
            return Ok(dt);
        }
    }

    Err(serde::de::Error::custom("Failed to parse date with any known format"))
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Wrapper {
        #[serde(with = "super")]
        date: jiff::civil::DateTime,
    }

    #[test]
    fn parses_both_formats() {
        for (input, expected) in [
            ("\"2023-09-24 13:06\"", "{\"date\":\"2023-09-24 13:06:00\"}"),
            ("\"2021-09-24 13:06:12\"", "{\"date\":\"2021-09-24 13:06:12\"}"),
        ] {
            let w: Wrapper =
                serde_saphyr::from_str(&format!("date: {}", input)).unwrap();
            assert_eq!(serde_json_like(&w), expected);
        }
    }

    #[test]
    fn rejects_invalid_dates() {
        assert!(serde_saphyr::from_str::<Wrapper>("date: \"not a date\"").is_err());
        assert!(serde_saphyr::from_str::<Wrapper>("date: \"2023-13-45 10:00\"").is_err());
    }

    fn serde_json_like(w: &Wrapper) -> String {
        format!("{{\"date\":\"{}\"}}", w.date.strftime(super::DATE_TIME_SECONDS_FORMAT))
    }
}
