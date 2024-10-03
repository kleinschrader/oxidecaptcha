use base64::{prelude::BASE64_STANDARD, Engine};
use bytes::Bytes;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::challenge::Prefix;

pub struct Solution {
    solution_bytes: Bytes
}

impl Solution {
    pub async fn validate(&self, prefix: &Prefix, difficulty: u8) -> bool{
        let mut hasher = Sha256::new();
        hasher.update(prefix.get_bytes());
        hasher.update(&self.solution_bytes);
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

        match last_byte_masked {
            0 => true,
            _ => false,
        }
    }
}

impl Serialize for Solution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let bytes_b64 = BASE64_STANDARD.encode(&self.solution_bytes);

        serializer.serialize_str(&bytes_b64)
    }
}

#[cfg(test)]
mod tests {
    use bytes::{BufMut, BytesMut};
    use futures::executor::block_on;
    use hex_literal::hex;
    use rand::{rngs::OsRng, Rng};

    use crate::challenge::Prefix;

    use super::Solution;

    #[test]
    #[ignore]
    fn find_fitting_test() {
        let prefix = bytes::Bytes::from_static(&hex!("12bedfcafb0491a1998f94f4648c494fc384ceec"));
        let prefix = Prefix::new(prefix);

        let mut rng = OsRng;

        let difficulty = 13;
        let solution_len = 12;

        let mut _found_solution = None;

        loop {
            let mut solution = BytesMut::with_capacity(12);

            (0..solution_len)
                .map(|_| rng.gen())
                .for_each(|b: u8| solution.put_u8(b));

            let solution = Solution{solution_bytes: solution.into()};
            
            if block_on(solution.validate(&prefix, difficulty)) {
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
        let solution_len = 12;

        let solution = bytes::Bytes::from_static(&hex!("d85ae00d155c6ca8edb4838a"));
        let solution = Solution { solution_bytes: solution };

        assert_eq!(block_on(solution.validate(&prefix, difficulty)), true);
    }

    #[test]
    fn test_invalid() {
        let prefix =  bytes::Bytes::from_static(&hex!("12bedfcafb0491a1998f94f4648c494fc384ceec"));
        let prefix = Prefix::new(prefix);

        let difficulty = 13;

        let solution = bytes::Bytes::from_static(&hex!("d85ae00e155c6ca8edb4838a"));
        let solution = Solution { solution_bytes: solution };

        assert_eq!(block_on(solution.validate(&prefix, difficulty)), false);
    }
}