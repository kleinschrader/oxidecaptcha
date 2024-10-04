use base64::{prelude::BASE64_STANDARD, Engine};
use bytes::Bytes;
use serde::{de, Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::challenge::Prefix;

#[derive(Debug)]
pub struct Solution (Bytes);

impl Solution {
    pub async fn _validate(&self, prefix: &Prefix, difficulty: u8) -> bool{
        let mut hasher = Sha256::new();
        hasher.update(prefix._get_bytes());
        hasher.update(&self.0);
        let bytes = hasher.finalize();

        let bytes_to_scan = (difficulty / 8) as usize;
        let bits_to_scan =( difficulty % 8) as usize;

        for i in 0..bytes_to_scan {
            if bytes[i] != 0 {
                return false;
            }
        };

        let last_byte = bytes[bytes_to_scan];
        
        let mask = !(0xFF >> bits_to_scan);
        let last_byte_masked = last_byte & mask;

        last_byte_masked == 0
    }
}

impl Serialize for Solution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let bytes_b64 = BASE64_STANDARD.encode(&self.0);

        serializer.serialize_str(&bytes_b64)
    }
}

impl<'de> Deserialize<'de> for Solution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
            let value = String::deserialize(deserializer)?;

            let base_data = BASE64_STANDARD.decode(value)
                .map_err(|_| de::Error::custom("could not base64 decode solution"))?;

            let bytes = Bytes::from(base_data);

            Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, Bytes, BytesMut};
    use futures::executor::block_on;
    use hex_literal::hex;
    use rand::{rngs::OsRng, Rng};
    use serde::Deserialize;

    use crate::challenge::Prefix;

    use super::Solution;

    #[test]
    #[ignore]
    fn find_fitting_test() {
        let prefix = bytes::Bytes::from_static(&hex!("12bedfcafb0491a1998f94f4648c494fc384ceec"));
        let prefix = Prefix::_new(prefix);

        let mut rng = OsRng;

        let difficulty = 13;
        let solution_len = 12;

        let mut _found_solution = None;

        loop {
            let mut solution = BytesMut::with_capacity(12);

            (0..solution_len)
                .map(|_| rng.gen())
                .for_each(|b: u8| solution.put_u8(b));

            let solution = Solution(solution.into());
            
            if block_on(solution._validate(&prefix, difficulty)) {
                _found_solution = Some(solution);
                break;
            }
        }

        let out = serde_json::to_string(&_found_solution.unwrap()).expect("Unable to convert to string");

        println!("{}", out);
    }

    #[test]
    fn test_valid() {
        let prefix =  bytes::Bytes::from_static(&hex!("12bedfcafb0491a1998f94f4648c494fc384ceec"));
        let prefix = Prefix::new(prefix);

        let difficulty = 13;

        let solution = bytes::Bytes::from_static(&hex!("d85ae00d155c6ca8edb4838a"));
        let solution = Solution ( solution );

        assert!(block_on(solution._validate(&prefix, difficulty)));
    }

    #[test]
    fn test_invalid() {
        let prefix =  bytes::Bytes::from_static(&hex!("12bedfcafb0491a1998f94f4648c494fc384ceec"));
        let prefix = Prefix::new(prefix);

        let difficulty = 13;

        let solution = bytes::Bytes::from_static(&hex!("d85ae00e155c6ca8edb4838a"));
        let solution = Solution ( solution );

        assert!(!block_on(solution._validate(&prefix, difficulty)));
    }

    #[test]
    fn test_deserialize() {
        let testee = r#"{"solution":"RkZnClxdPgo="}"#;

        #[derive(Debug, Deserialize)]
        struct TestStruct {
            solution: Solution
        }

        let r: TestStruct = serde_json::from_str(testee)
            .expect("Unable to parse test string");

        let bytes = Bytes::from_static( &hex!("4646670a5c5d3e0a"));

        assert_eq!(r.solution.0, bytes);
    }


    #[test]
    fn test_deserialize_not_base64() {
        let testee = r#"{"_solution":"RkZnCxdPo="}"#;

        #[derive(Debug, Deserialize)]
        struct TestStruct {
            _solution: Solution
        }

        let e = serde_json::from_str::<TestStruct>(testee)
            .expect_err("Somehow we were able parse test string")
            .to_string();

        assert_eq!(e, "could not base64 decode solution at line 1 column 26");
    }
}