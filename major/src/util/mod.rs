use serde::{Deserialize, Deserializer};

pub mod raw_timestamp {
    use chrono::{TimeZone, DateTime, Utc};
    use serde::{Deserialize, Deserializer};

    use crate::DEFAULT_TIMEZONE_OFFSET;

    /* pub fn serialize<S: Serializer>(datetime: &str, serializer: S) -> Result<S::Ok, S::Error> {
        DEFAULT_TIMEZONE_OFFSET
            .timestamp(datetime.parse::<i64>().unwrap(), 0)
            .to_rfc3339()
            .serialize(serializer)
    } */

    /* pub fn deserialize_from_str<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = String::deserialize(d)?.parse::<i64>().unwrap_or(0);
        Ok(from_timestamp(timestamp))
    } */

    pub fn deserialize_from_int<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(d)?;
        Ok(from_timestamp(timestamp))
    }

    fn from_timestamp(timestamp: i64) -> DateTime<Utc> {
        DEFAULT_TIMEZONE_OFFSET.timestamp(timestamp, 0).with_timezone(&Utc)
    }

    /* pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = String::deserialize(d)?.parse::<i64>().unwrap_or(0);
        Ok(DEFAULT_TIMEZONE_OFFSET.timestamp(timestamp, 0).into())
    } */

    pub fn optional_deserialize_from_number<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(d)?;
        Ok(Some(from_timestamp(timestamp)))
    }
}

pub mod local_timestamp {
    use chrono::{DateTime, Utc, Local};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> {
        datetime.with_timezone(&Local)
            .to_rfc3339()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>,
    {
        DateTime::<Utc>::deserialize(d)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
enum ExtraDataLayer<T>
where T: Default {
    Data(#[serde(default)] T),
    Embedded {
        #[serde(default)]
        data: T
    }
}

impl<T> ExtraDataLayer<T>
where T: Default {
    fn get_data(self) -> T {
        match self {
            ExtraDataLayer::Data(data) => data,
            ExtraDataLayer::Embedded { data } => data,
        }
    }
}

pub fn unwrap_one_layer_of_data<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    ExtraDataLayer::<T>::deserialize(d).map(|data| data.get_data())
}

pub fn lossy_deserialize_usize<'de, D>(d: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(d)?;
    Ok(s.parse::<usize>().unwrap_or(0))
}

pub fn number_to_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = usize::deserialize(d)?;
    Ok(s == 1)
}

// https://github.com/Mingun/ksc-rs/blob/8532f701e660b07b6d2c74963fdc0490be4fae4b/src/parser.rs#L18-L42

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
  /// Single value
  One(T),
  /// Array of values
  Vec(Vec<T>),
}

impl<T> Default for OneOrMany<T> {
    fn default() -> Self {
        Self::Vec(vec![])
    }
}

impl<T> From<OneOrMany<T>> for Vec<T> {
  fn from(from: OneOrMany<T>) -> Self {
    match from {
      OneOrMany::One(val) => vec![val],
      OneOrMany::Vec(vec) => vec,
    }
  }
}
