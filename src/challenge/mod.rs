use serde::{ser::SerializeStruct, Serialize};
use timestamp::Timestamp;
use uuid::Uuid;
pub use prefix::Prefix;


mod prefix;
mod timestamp;

#[derive(Debug, Clone)]
pub struct Challenge {
    id: Uuid,
    prefixes: Vec<Prefix>,
    difficulty: u8,
    challenges_to_solve: usize,
    solution_length: usize,
    expires_at: Timestamp
}

impl Challenge {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

impl Serialize for Challenge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Challenge", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("prefixes", &self.prefixes)?;
        state.serialize_field("difficulty", &self.difficulty)?;
        state.serialize_field("challegesToSolve", &self.challenges_to_solve)?;
        state.serialize_field("solutionLength", &self.solution_length)?;
        state.serialize_field("expiresAt", &self.expires_at)?;
        state.end()
    }
}