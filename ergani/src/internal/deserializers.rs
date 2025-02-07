use chrono::{DateTime, Utc};
use serde::de;
use serde::de::Visitor;

pub(crate) fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct DateTimeVisitor;

    impl Visitor<'_> for DateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a valid datetime string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            DateTime::parse_from_rfc3339(value)
                .map_err(|_| E::custom("failed to parse datetime"))
                .map(|dt| dt.with_timezone(&Utc))
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&value)
        }
    }

    deserializer.deserialize_str(DateTimeVisitor)
}
