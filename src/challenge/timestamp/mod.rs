use std::time::{Duration, SystemTime};

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Timestamp(SystemTime);

impl From<SystemTime> for Timestamp {
    fn from(value: SystemTime) -> Self {
        Timestamp(value)
    }
}

impl From<u64> for Timestamp {
    fn from(value: u64) -> Self {
        let duration = Duration::from_secs(value);

        let inner = SystemTime::UNIX_EPOCH + duration;

        Self(inner)
    }
}

impl From<Timestamp> for u64 {
    fn from(value: Timestamp) -> Self {
        u64::from(&value)
    }
}

impl From<&Timestamp> for u64 {
    fn from(value: &Timestamp) -> Self {
        value
            .0
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Duration since failed")
            .as_secs()
    }
}

impl Timestamp {
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now();
        self.0 < now
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.into())
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use super::Timestamp;

    #[test]
    fn test_from_u64() {
        let testee = 12;

        let result = Timestamp::from(testee);

        assert_eq!(
            result.0.duration_since(SystemTime::UNIX_EPOCH).unwrap(),
            Duration::from_secs(12)
        )
    }

    #[test]
    fn test_into_u64() {
        let sometime = SystemTime::UNIX_EPOCH + Duration::from_secs(12);

        let testee = Timestamp(sometime);

        assert_eq!(u64::from(testee), 12)
    }

    #[test]
    fn test_serialize() {
        let sometime = SystemTime::UNIX_EPOCH + Duration::from_secs(12);

        let testee = Timestamp(sometime);

        let json = serde_json::json! ( {"testee": testee}).to_string();

        assert_eq!(json, r#"{"testee":12}"#)
    }
}
