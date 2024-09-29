use std::{io::Write, time::Duration};

use sha2::{Digest, Sha256};
use uuid::Uuid;

mod deserialize;
mod lifetime;

#[derive(Debug, Clone)]
pub struct Site {
    id: Uuid,
    api_key: String,
    api_key_hash: Vec<u8>,
    prefixes: usize,
    difficulty: u8,
    solution_length: usize,
    lifetime: Duration
}

impl Site {
    pub fn new(id: Uuid, api_key: String, prefixes: usize, difficulty: u8, solution_length: usize, lifetime: Duration) -> Self {
        let mut hasher = Sha256::new();
        hasher.write_all(&api_key.as_bytes()).expect("Unable to hash api_key");
        let api_key_hash = hasher.finalize().to_vec();

        Self {
            id,
            api_key,
            api_key_hash,
            prefixes,
            difficulty,
            solution_length,
            lifetime
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use hex_literal::hex;
    use uuid::uuid;

    use super::Site;

    #[test]
    fn test_deserialize() {
        let test_string = 
        r#"
            {
                "id": "60601796-7dc2-4d4f-afae-5728592bba6f",
                "apiKey": "cool",
                "difficulty": 17,
                "prefixes": 12,
                "solutionLength": 21,
                "lifetime": {
                    "minutes": 2
                }
            }
        "#;

        let test = serde_json::from_str::<Site>(test_string).expect("Failed parsing json");

        assert_eq!(test.id, uuid!("60601796-7dc2-4d4f-afae-5728592bba6f"));
        assert_eq!(test.api_key, "cool");
        assert_eq!(test.api_key_hash, hex!("c34045c1a1db8d1b3fca8a692198466952daae07eaf6104b4c87ed3b55b6af1b"));
        assert_eq!(test.prefixes, 12);
        assert_eq!(test.solution_length, 21);
        assert_eq!(test.lifetime, Duration::from_secs(120));


    }
}