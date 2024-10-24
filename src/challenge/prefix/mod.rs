use base64::prelude::*;
use bytes::{BufMut, Bytes, BytesMut};
use serde::Serialize;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Prefix(Bytes);

impl Serialize for Prefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let encoded = BASE64_STANDARD.encode(&self.0);

        serializer.serialize_str(&encoded)
    }
}

impl Prefix {
    pub fn _new(bytes: Bytes) -> Self {
        Self(bytes)
    }

    pub fn generate(size: usize) -> Self {
        let mut bytes = BytesMut::with_capacity(size);

        let mut rng = thread_rng();

        (0..size)
            .map(|_| rng.gen::<u8>())
            .for_each(|d| bytes.put_u8(d));

        Prefix(bytes.into())
    }

    pub fn _get_bytes(&self) -> &Bytes {
        &self.0
    }

    pub fn write_to_buf(&self, target: &mut impl BufMut) {
        target.put(self.0.as_ref());
    }

    pub fn get_size_hint(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use serde_json::json;

    use super::Prefix;

    #[test]
    fn test_generate() {
        let prefix = Prefix::generate(8);

        assert_eq!(prefix.0.len(), 8)
    }

    #[test]
    fn test_serialize() {
        let data = Bytes::from_static(&[12, 14, 43, 50, 90]);
        let prefix = Prefix(data);

        let json = json!({"value": prefix}).to_string();

        assert_eq!(json, r#"{"value":"DA4rMlo="}"#)
    }
}
