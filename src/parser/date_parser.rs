use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const DATE_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M";
const DATE_TIME_SECONDS_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(DATE_TIME_SECONDS_FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let formats = [DATE_TIME_SECONDS_FORMAT, DATE_TIME_FORMAT];

    for format in formats.iter() {
        if let Ok(dt) = NaiveDateTime::parse_from_str(&s, format) {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc));
        }
    }

    Err(serde::de::Error::custom("Failed to parse date with any known format"))
}
