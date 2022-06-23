pub mod iso8601 {
    use serde::{Serializer, Serialize, ser::Error};
    use time::{serde::iso8601, OffsetDateTime, format_description::well_known::{Iso8601, iso8601::{Config, TimePrecision}}};

    pub use iso8601::deserialize;

    const SERDE_CONFIG: u128 = Config::DEFAULT.set_time_precision(TimePrecision::Second { decimal_digits: None }).encode();

    pub fn serialize<S: Serializer>(
        datetime: &OffsetDateTime, 
        serializer: S
    ) -> Result<S::Ok, S::Error> {
        datetime
            .format(&Iso8601::<SERDE_CONFIG>)
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}
