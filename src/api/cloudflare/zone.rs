use crate::api::cloudflare::api_result::ApiResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Zone {
    id: String,
}

impl Zone {
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
impl Zone {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl ApiResult for Zone {}
