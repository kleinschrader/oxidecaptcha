use std::fmt;

use kale_duration::AbsoluteDuration;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use super::Site;

impl<'de> Deserialize<'de> for Site {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Id,
            ApiKey,
            Prefixes,
            PrefixesToSolve,
            PrefixLength,
            Difficulty,
            SolutionLength,
            Lifetime,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`id`, `apiKey`, `prefixLength`, `prefixes`, `prefixesToSolve`, `difficulty`, `solutionLength or `liftime`")
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
                            "prefixesToSolve" => Ok(Field::PrefixesToSolve),
                            "prefixLength" => Ok(Field::PrefixLength),
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
                let mut prefixes_to_solve = None;
                let mut prefix_length = None;
                let mut solution_length = None;
                let mut lifetime = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::ApiKey => {
                            if api_key.is_some() {
                                return Err(de::Error::duplicate_field("apiKey"));
                            }
                            api_key = Some(map.next_value()?);
                        }
                        Field::Difficulty => {
                            if difficulty.is_some() {
                                return Err(de::Error::duplicate_field("difficulty"));
                            }
                            difficulty = Some(map.next_value()?);
                        }
                        Field::Prefixes => {
                            if prefixes.is_some() {
                                return Err(de::Error::duplicate_field("prefixes"));
                            }
                            prefixes = Some(map.next_value()?);
                        }
                        Field::PrefixLength => {
                            if prefix_length.is_some() {
                                return Err(de::Error::duplicate_field("prefixLength"));
                            }

                            prefix_length = Some(map.next_value()?);
                        }
                        Field::PrefixesToSolve => {
                            if prefixes_to_solve.is_some() {
                                return Err(de::Error::duplicate_field("prefixesToSolve"));
                            }

                            prefixes_to_solve = Some(map.next_value()?);
                        }
                        Field::SolutionLength => {
                            if solution_length.is_some() {
                                return Err(de::Error::duplicate_field("solutionLength"));
                            }
                            solution_length = Some(map.next_value()?);
                        }
                        Field::Lifetime => {
                            if lifetime.is_some() {
                                return Err(de::Error::duplicate_field("lifetime"));
                            }
                            lifetime = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
                let api_key = api_key.ok_or_else(|| de::Error::missing_field("apiKey"))?;
                let difficulty =
                    difficulty.ok_or_else(|| de::Error::missing_field("difficulty"))?;
                let prefixes = prefixes.ok_or_else(|| de::Error::missing_field("prefixes"))?;
                let prefix_length =
                    prefix_length.ok_or_else(|| de::Error::missing_field("prefixLength"))?;
                let prefixes_to_solve =
                    prefixes_to_solve.ok_or_else(|| de::Error::missing_field("prefixesToSolve"))?;
                let solution_length =
                    solution_length.ok_or_else(|| de::Error::missing_field("solutionLength"))?;
                let lifetime: AbsoluteDuration =
                    lifetime.ok_or_else(|| de::Error::missing_field("lifetime"))?;

                Ok(Site::new(
                    id,
                    api_key,
                    prefixes,
                    prefix_length,
                    prefixes_to_solve,
                    difficulty,
                    solution_length,
                    lifetime.into(),
                ))
            }
        }

        const FIELDS: &[&str] = &[
            "`id`",
            "`apiKey`",
            "`prefixes`",
            "`prefixes_to_solve`",
            "`difficulty`",
            "`solutionLength`",
            "`lifetime`",
        ];
        deserializer.deserialize_struct("Duration", FIELDS, SiteVisitor)
    }
}
