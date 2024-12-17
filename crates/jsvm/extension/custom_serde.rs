pub mod serde_option_duration_ms {
    //! Serializing and deserializing `Option<Duration>` as milliseconds.
    //!
    //! # Example:
    //!
    //! - `{"duration": 1500}` for `Some(Duration::from_millis(1500))`
    //! - `{"duration": null}` for `None`

    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes an `Option<Duration>` into milliseconds.
    pub fn serialize<S>(option: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match option {
            Some(duration) => serializer.serialize_some(&(duration.as_millis() as u64)),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes an `Option<Duration>` from milliseconds.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis: Option<u64> = Option::deserialize(deserializer)?;
        Ok(millis.map(Duration::from_millis))
    }
}

pub mod serde_duration_ms {
    //! Serializing and deserializing `Duration` as milliseconds.
    //!
    //! # Example:
    //!
    //! - `{"duration": 1500}` for `Duration::from_millis(1500)`

    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serializes a `Duration` into milliseconds.
    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let millis = duration.as_millis() as u64;
        millis.serialize(serializer)
    }

    /// Deserializes a `Duration` from milliseconds.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}
