/// de/serialize sketchware's date format into a timestamp using chrono
pub(crate) mod date_to_timestamp {
    use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
    use serde::{de, Deserialize, Serializer};
    use serde::de::Error;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error> where D: de::Deserializer<'de> {
        let v = String::deserialize(deserializer)?;

        let year = v[0..4].parse::<i32>().map_err(D::Error::custom)?;
        let month = v[4..6].parse::<u32>().map_err(D::Error::custom)?;
        let day = v[6..8].parse::<u32>().map_err(D::Error::custom)?;
        let hour = v[8..10].parse::<u32>().map_err(D::Error::custom)?;
        let minute = v[10..12].parse::<u32>().map_err(D::Error::custom)?;
        let second = v[12..14].parse::<u32>().map_err(D::Error::custom)?;

        Ok(
            NaiveDate::from_ymd(year, month, day)
                .and_hms(hour, minute, second)
                .timestamp() as u64
        )
    }

    pub fn serialize<S>(timestamp: &u64, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let date = NaiveDateTime::from_timestamp(*timestamp as i64, 0);
        let date_str = format!(
            "{}{}{}{}{}{}",
            date.year(),
            date.month(),
            date.day(),
            date.hour(),
            date.minute(),
            date.second()
        );

        serializer.serialize_str(date_str.as_str())
    }
}

// modules used by serde to de/serialize string numbers into regular numbers
//
// can't globally define a generic type for each functions in a module, so I had to do this ¯\_(ツ)_/¯
pub(crate) mod string_to_u16 {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;

    pub fn serialize<S>(num: &u16, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(num.to_string().as_str())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error> where D: Deserializer<'de> {
        String::deserialize(deserializer)?.parse().map_err(|_|D::Error::custom("a number"))
    }
}