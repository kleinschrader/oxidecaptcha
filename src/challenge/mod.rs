use std::time::SystemTime;

use anyhow::Result;
use axum::response::{IntoResponse, Response};
use serde::{ser::SerializeStruct, Serialize};
use timestamp::Timestamp;
use uuid::Uuid;

pub use prefix::Prefix;

use crate::site::Site;

mod prefix;
mod timestamp;

#[derive(Debug, Clone)]
pub struct Challenge<'site, T: Clone> {
    id: Uuid,
    prefixes: Vec<Prefix>,
    expires_at: Timestamp,
    site: &'site T,
}

impl<'site> Challenge<'site, Site> {
    pub fn generate(site: &'site Site) -> Self {
        let id = Uuid::new_v4();

        let prefixes = (0..site.get_prefix_count())
            .map(|_| Prefix::generate(site.get_prefix_length()))
            .collect();

        let expires_at = SystemTime::now();
        let expires_at = expires_at + *site.get_lifetime();
        let expires_at = expires_at.into();
        Challenge {
            id,
            prefixes,
            expires_at,
            site,
        }
    }
}

impl<'site, T: Clone> Challenge<'site, T> {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_prefix(&self, n: usize) -> Option<&Prefix> {
        self.prefixes.get(n)
    }
}

impl<'site> Challenge<'site, Site> {
    pub fn pluck(self) -> Challenge<'static, ()> {
        Challenge {
            id: self.id,
            prefixes: self.prefixes,
            expires_at: self.expires_at,
            site: &(),
        }
    }
}

impl<'site> Challenge<'static, ()> {
    pub fn unpluck(self, site: &'site Site) -> Challenge<'site, Site> {
        Challenge {
            id: self.id,
            prefixes: self.prefixes,
            expires_at: self.expires_at,
            site,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at.is_expired()
    }
}

impl<'site> Serialize for Challenge<'site, Site> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Challenge", 6)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("prefixes", &self.prefixes)?;
        state.serialize_field("difficulty", &self.site.get_difficulty())?;
        state.serialize_field("challegesToSolve", &self.site.get_prefixes_to_solve())?;
        state.serialize_field("solutionLength", &self.site.get_solution_length())?;
        state.serialize_field("expiresAt", &self.expires_at)?;
        state.end()
    }
}

impl<'site> IntoResponse for Challenge<'site, Site> {
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
