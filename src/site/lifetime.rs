use std::{fmt, time::Duration};

use serde::{de::{self, MapAccess, Visitor}, Deserialize, Deserializer};

#[derive(Debug)]
pub struct Lifetime {
    inner: Duration
}

impl From<Lifetime> for Duration {
    fn from(value: Lifetime) -> Self {
        value.inner
    }
}

impl<'de> Deserialize<'de> for Lifetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Seconds, Minutes }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`seconds` or `minutes`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "seconds" => Ok(Field::Seconds),
                            "minutes" => Ok(Field::Minutes),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct LifetimeVisitor;

        impl<'de> Visitor<'de> for LifetimeVisitor {
            type Value = Lifetime;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Lifetime")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Lifetime, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut secs = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Seconds => {
                            if secs.is_some() {
                                return Err(de::Error::custom("Either seconds or minutes"));
                            }
                            secs = Some(map.next_value()?);
                        }
                        Field::Minutes => {
                            if secs.is_some() {
                                return Err(de::Error::custom("Either seconds or minutes"));
                            }

                            secs = Some(map.next_value().map(|v: u64| v * 60)?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("seconds or minutes"))?;

                let duration = Duration::from_secs(secs);
                Ok(Lifetime { inner: duration })
            }
        }

        const FIELDS: &[&str] = &["seconds", "minutes"];
        deserializer.deserialize_struct("Duration", FIELDS, LifetimeVisitor)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::Lifetime;

    #[test]
    fn test_secs() {
        let testee = r#"{"seconds":32}"#;

        let lifetime: Lifetime = serde_json::from_str(testee).expect("Unable to parse test case");

        assert_eq!(
            lifetime.inner,
            Duration::from_secs(32)
        )
    }

    #[test]
    fn test_mins() {
        let testee = r#"{"minutes":2}"#;

        let lifetime: Lifetime = serde_json::from_str(testee).expect("Unable to parse test case");

        assert_eq!(
            lifetime.inner,
            Duration::from_secs(120)
        )
    }

    #[test]
    fn test_dual_fields() {
        let testee = r#"{"minutes":2,"seconds":21}"#;

        let _ = serde_json::from_str::<Lifetime>(testee).expect_err("Somehow parsing succeded");
    }
}