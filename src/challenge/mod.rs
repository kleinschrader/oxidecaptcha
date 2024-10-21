use std::time::SystemTime;

use anyhow::Result;
use axum::response::{IntoResponse, Response};
use serde::{ser::SerializeStruct, Serialize};
use timestamp::Timestamp;
use siteparameter::SiteParameter;
use uuid::Uuid;

pub use prefix::Prefix;

use crate::site::Site;

mod prefix;
mod timestamp;
mod siteparameter;

#[derive(Debug, Clone)]
pub struct Challenge {
    id: Uuid,
    prefixes: Vec<Prefix>,
    expires_at: Timestamp,
    site_parameter: SiteParameter
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

        let site_parameter = SiteParameter{
            difficulty: site.get_difficulty(),
            prefixes_to_solve: site.get_prefixes_to_solve(),
            solution_length: site.get_solution_length()
        };

        Challenge {
            id,
            prefixes,
            expires_at,
            site_parameter,
        }
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
}


impl Serialize for Challenge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Challenge", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("prefixes", &self.prefixes)?;
        state.serialize_field("difficulty", &self.site_parameter.difficulty )?;
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
