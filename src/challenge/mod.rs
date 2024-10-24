use std::{io::Read, mem::MaybeUninit, time::SystemTime};

use anyhow::Result;
use axum::response::{IntoResponse, Response};
use bytes::{buf::Reader, Buf, BufMut, Bytes, BytesMut};
use parsingerror::ParsingError;
use serde::{ser::SerializeStruct, Serialize};
use siteparameter::SiteParameter;
use timestamp::Timestamp;
use tracing::warn;
use uuid::Uuid;

pub use prefix::Prefix;

use crate::site::Site;

mod parsingerror;
mod prefix;
mod siteparameter;
mod timestamp;

const MAGIC_BYTES: [u8; 3] = [0x12, 0x0A, 0x01];

#[derive(Debug, Clone)]
pub struct Challenge {
    id: Uuid,
    prefixes: Vec<Prefix>,
    expires_at: Timestamp,
    site_parameter: SiteParameter,
}

impl Challenge {
    pub fn generate(site: &Site) -> Self {
        let id = Uuid::new_v4();

        let prefixes = (0..site.get_prefix_count())
            .map(|_| Prefix::generate(site.get_prefix_length()))
            .collect();

        let expires_at = SystemTime::now();
        let expires_at = expires_at + *site.get_lifetime();
        let expires_at = expires_at.into();

        let site_parameter = SiteParameter {
            difficulty: site.get_difficulty(),
            prefixes_to_solve: site.get_prefixes_to_solve(),
            solution_length: site.get_solution_length(),
        };

        Challenge {
            id,
            prefixes,
            expires_at,
            site_parameter,
        }
    }

    pub fn parse(mut data: Bytes) -> Result<(), ParsingError> {
        let mut reader = data.reader();

        let mut magic_bytes: [u8; 3] = [0; 3];

        reader
            .read_exact(&mut magic_bytes)
            .map_err(|e: std::io::Error| match e.kind() {
                std::io::ErrorKind::UnexpectedEof => ParsingError::UnexpectedEnd,
                _ => ParsingError::UnexpectedError(e.to_string()),
            })?;

        if magic_bytes != MAGIC_BYTES {
            return Err(ParsingError::MissmatchedMagicBytes {
                expected: MAGIC_BYTES,
                found: magic_bytes,
            });
        }

        Ok(())
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_prefix(&self, n: usize) -> Option<&Prefix> {
        self.prefixes.get(n)
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.is_expired()
    }

    /// Bytes are in the following format
    /// ```text
    /// | 0x0 | 0x1 | 0x2 | 0x3 | 0x4 | 0x5 | 0x6 | 0x7 | 0x8 | 0x9 | 0xA | 0xB | 0xC | 0xD | 0xE | 0xF |
    /// | magic bytes     |  challange-uuid                                                             |
    /// |                 |  exires at timestamp                          | diff| prefix count          |
    /// |                       | prefixes to solve                             | solution length       |
    /// |                       | prefix size                                   | prefixes...           |
    /// ```
    pub fn to_bytes(&self) -> Option<Bytes> {
        const BYTES_SITE_HINT: usize = MAGIC_BYTES.len()
            + size_of::<u128>() // CH UUID
            + size_of::<u64>()  // EXP TS
            + size_of::<u8>()   // Diff
            + size_of::<u64>()  // PREFIX COUNT
            + size_of::<u64>()  // PREFIX TO SOLVE
            + size_of::<u64>()  // SOLUTION LENGHT
            + size_of::<u64>(); // PREFIX SIZE

        let prefix_length = self.prefixes.get(0).map(|p| p.get_size_hint());

        let prefix_length = match prefix_length {
            Some(r) => r,
            None => {
                warn!("Trying to serialize bytes from no prefixes");
                return None;
            }
        };

        let prefixs_bytes = self.prefixes.len() * prefix_length;

        let mut buffer = BytesMut::with_capacity(prefixs_bytes);

        buffer.put(&MAGIC_BYTES as &[u8]);
        buffer.put(self.id.as_bytes() as &[u8]);
        self.expires_at.write_to_buf(&mut buffer);
        buffer.put_u8(self.site_parameter.difficulty);
        buffer.put_u64_le(self.prefixes.len() as u64);
        buffer.put_u64_le(self.site_parameter.prefixes_to_solve as u64);
        buffer.put_u64_le(self.site_parameter.solution_length as u64);
        buffer.put_u64_le(prefix_length as u64);

        self.prefixes
            .iter()
            .for_each(|e| e.write_to_buf(&mut buffer));

        Some(buffer.into())
    }
}

impl Serialize for Challenge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Challenge", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("prefixes", &self.prefixes)?;
        state.serialize_field("difficulty", &self.site_parameter.difficulty)?;
        state.serialize_field("challegesToSolve", &self.site_parameter.prefixes_to_solve)?;
        state.serialize_field("solutionLength", &self.site_parameter.solution_length)?;
        state.serialize_field("expiresAt", &self.expires_at)?;
        state.end()
    }
}

impl IntoResponse for Challenge {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self)
            .expect("Unable to serialize boyd")
            .into();

        Response::builder()
            .header("Content-Type", "application/json")
            .body(body)
            .expect("Unable ot create body")
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use hex_literal::hex;
    use uuid::uuid;

    use crate::challenge::{parsingerror::ParsingError, MAGIC_BYTES};

    use super::{siteparameter::SiteParameter, timestamp::Timestamp, Challenge, Prefix};

    const HEX_STRING: [u8; 68] = hex!("120a01a6da9fe3be28482bb913b2a88788b071bf4b1a67000000000C0200000000000000300000000000000010000000000000000400000000000000DEADBEEFDDB8FFAA");

    #[test]
    fn test_to_bytes() {
        //a6da9fe3be28482bb913b2a88788b071
        let id = uuid!("a6da9fe3-be28-482b-b913-b2a88788b071");

        let prefixes = vec![
            Prefix::_new(Bytes::from_static(&hex!("DEADBEEF"))),
            Prefix::_new(Bytes::from_static(&hex!("DDB8FFAA"))),
        ];

        let expires_at = Timestamp::from(1729776575u64);

        let site_parameter = SiteParameter {
            difficulty: 0xC,
            prefixes_to_solve: 0x30,
            solution_length: 0x10,
        };

        let challenge = Challenge {
            id,
            prefixes,
            expires_at,
            site_parameter,
        };

        let r = challenge.to_bytes().unwrap();

        assert_eq!(r.as_ref(), HEX_STRING);
    }

    #[test]
    fn test_parse() {
        let bytes = Bytes::from_static(&HEX_STRING);

        let challange = Challenge::parse(bytes).unwrap();
    }

    #[test]
    fn test_wrong_magic_bytes() {
        const TEST_BYTES: [u8; 3] = [0xAB, 0xF1, 0x92];
        let _binding = TEST_BYTES.clone();

        let bytes = Bytes::from_static(&TEST_BYTES);

        let r = Challenge::parse(bytes).expect_err("Somehow parsing challange succeeded");

        assert!(matches!(
            r,
            ParsingError::MissmatchedMagicBytes {
                expected: MAGIC_BYTES,
                found: _binding
            }
        ))
    }
}
