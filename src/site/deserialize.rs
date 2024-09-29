use std::fmt;

use serde::{de::{self, MapAccess, Visitor}, Deserialize, Deserializer};

use crate::site::lifetime::Lifetime;

use super::Site;


impl<'de> Deserialize<'de> for Site {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Id, ApiKey, Prefixes, Difficulty, SolutionLength, Lifetime }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`id`, `apiKey`, `prefixes`, `difficulty`, `solutionLength or `liftime`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "id" => Ok(Field::Id),
                            "apiKey" => Ok(Field::ApiKey),
                            "difficulty" => Ok(Field::Difficulty),
                            "prefixes" => Ok(Field::Prefixes),
                            "solutionLength" => Ok(Field::SolutionLength),
                            "lifetime" => Ok(Field::Lifetime),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct SiteVisitor;

        impl<'de> Visitor<'de> for SiteVisitor {
            type Value = Site;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct site")
            }
            
            fn visit_map<V>(self, mut map: V) -> Result<Site, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut api_key = None;
                let mut difficulty = None;
                let mut prefixes = None;
                let mut solution_length = None;
                let mut lifetime = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        },
                        Field::ApiKey => {
                            if api_key.is_some() {
                                return Err(de::Error::duplicate_field("apiKey"));
                            }
                            api_key = Some(map.next_value()?);
                        },
                        Field::Difficulty => {
                            if difficulty.is_some() {
                                return Err(de::Error::duplicate_field("difficulty"));
                            }
                            difficulty = Some(map.next_value()?);
                        },
                        Field::Prefixes => {
                            if prefixes.is_some() {
                                return Err(de::Error::duplicate_field("prefixes"));
                            }
                            prefixes = Some(map.next_value()?);
                        },
                        Field::SolutionLength => {
                            if solution_length.is_some() {
                                return Err(de::Error::duplicate_field("solutionLength"));
                            }
                            solution_length = Some(map.next_value()?);
                        },
                        Field::Lifetime => {
                            if lifetime.is_some() {
                                return Err(de::Error::duplicate_field("lifetime"));
                            }
                            lifetime = Some(map.next_value()?);
                        },
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let api_key = api_key.ok_or_else(|| de::Error::missing_field("apiKey"))?;
                let difficulty = difficulty.ok_or_else(|| de::Error::missing_field("difficulty"))?;
                let prefixes = prefixes.ok_or_else(|| de::Error::missing_field("prefixes"))?;
                let solution_length = solution_length.ok_or_else(|| de::Error::missing_field("solutionLength"))?;
                let lifetime: Lifetime = lifetime.ok_or_else(|| de::Error::missing_field("lifetime"))?;

               

                Ok(Site::new(id, api_key, prefixes, difficulty, solution_length, lifetime.into()))
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, SiteVisitor)
    }
}