use chrono::{DateTime, TimeZone, Utc};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

pub const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Snowflake {
    pub raw: u64,
    pub timestamp_ms: u64,
    pub worker_id: u8,
    pub process_id: u8,
    pub increment: u16,
}

#[derive(Debug)]
pub enum SnowflakeError {
    EmptyId,
    InvalidId(ParseIntError),
    TimestampOutOfRange(u64),
}

impl fmt::Display for SnowflakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnowflakeError::EmptyId => write!(f, "snowflake id is empty"),
            SnowflakeError::InvalidId(error) => write!(f, "invalid snowflake id: {error}"),
            SnowflakeError::TimestampOutOfRange(value) => {
                write!(f, "timestamp is out of range for chrono: {value}")
            }
        }
    }
}

impl Error for SnowflakeError {}

impl Snowflake {
    pub fn from_raw(raw: u64) -> Self {
        Self {
            raw,
            timestamp_ms: discord_timestamp_ms(raw),
            worker_id: ((raw >> 17) & 0x1F) as u8,
            process_id: ((raw >> 12) & 0x1F) as u8,
            increment: (raw & 0xFFF) as u16,
        }
    }

    pub fn as_u64(self) -> u64 {
        self.raw
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.raw.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeVisitor;

        impl<'de> Visitor<'de> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a discord snowflake id as string or unsigned integer")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                parse_id(value).map(Snowflake::from_raw).map_err(E::custom)
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Snowflake::from_raw(value))
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

pub trait Timestamp {
    fn to_chrono(self) -> Result<DateTime<Utc>, SnowflakeError>;
}

impl Timestamp for Snowflake {
    fn to_chrono(self) -> Result<DateTime<Utc>, SnowflakeError> {
        timestamp_to_chrono(self.timestamp_ms)
    }
}

fn parse_id(id: &str) -> Result<u64, SnowflakeError> {
    let trimmed = id.trim();
    if trimmed.is_empty() {
        return Err(SnowflakeError::EmptyId);
    }

    trimmed
        .parse::<u64>()
        .map_err(SnowflakeError::InvalidId)
}

fn discord_timestamp_ms(id: u64) -> u64 {
    (id >> 22) + DISCORD_EPOCH_MS
}

fn timestamp_to_chrono(timestamp_ms: u64) -> Result<DateTime<Utc>, SnowflakeError> {
    let timestamp_ms =
        i64::try_from(timestamp_ms).map_err(|_| SnowflakeError::TimestampOutOfRange(timestamp_ms))?;

    Utc.timestamp_millis_opt(timestamp_ms)
        .single()
        .ok_or(SnowflakeError::TimestampOutOfRange(timestamp_ms as u64))
}

#[cfg(test)]
mod tests {
    use super::{DISCORD_EPOCH_MS, Snowflake, SnowflakeError, Timestamp, parse_id, timestamp_to_chrono};
    use serde::Deserialize;

    #[test]
    fn parses_snowflake_id() {
        let parsed = parse_id("1480908521355874518");
        match parsed {
            Ok(value) => assert_eq!(value, 1_480_908_521_355_874_518),
            Err(error) => panic!("expected valid snowflake id: {error}"),
        }
    }

    #[test]
    fn rejects_empty_id() {
        let error = parse_id(" ");
        assert!(matches!(error, Err(SnowflakeError::EmptyId)));
    }

    #[test]
    fn decodes_snowflake_fields() {
        let value = Snowflake::from_raw(1_480_908_521_355_874_518);
        assert_eq!(value.timestamp_ms, 1_773_146_505_441);

        let rebuilt = ((value.timestamp_ms - DISCORD_EPOCH_MS) << 22)
            | ((value.worker_id as u64) << 17)
            | ((value.process_id as u64) << 12)
            | (value.increment as u64);
        assert_eq!(rebuilt, value.raw);
    }

    #[test]
    fn converts_snowflake_to_chrono() {
        let value = Snowflake::from_raw(1_480_908_521_355_874_518);
        let datetime = match value.to_chrono() {
            Ok(value) => value,
            Err(error) => panic!("expected valid chrono conversion: {error}"),
        };

        assert_eq!(datetime.timestamp_millis(), value.timestamp_ms as i64);
    }

    #[test]
    fn rejects_out_of_range_timestamp_for_chrono() {
        let timestamp = (i64::MAX as u64) + 1;
        let result = timestamp_to_chrono(timestamp);

        assert!(matches!(
            result,
            Err(SnowflakeError::TimestampOutOfRange(value)) if value == timestamp
        ));
    }

    #[derive(Deserialize)]
    struct IdHolder {
        id: Snowflake,
    }

    #[test]
    fn deserializes_snowflake_id_from_string() {
        let holder = serde_json::from_str::<IdHolder>(r#"{"id":"1480908521355874518"}"#);
        match holder {
            Ok(value) => assert_eq!(value.id.raw, 1_480_908_521_355_874_518),
            Err(error) => panic!("expected valid snowflake id string: {error}"),
        }
    }

    #[test]
    fn deserializes_snowflake_id_from_number() {
        let holder = serde_json::from_str::<IdHolder>(r#"{"id":1480908521355874518}"#);
        match holder {
            Ok(value) => assert_eq!(value.id.raw, 1_480_908_521_355_874_518),
            Err(error) => panic!("expected valid snowflake id number: {error}"),
        }
    }
}
