use crate::api::cloudflare::api_result::ApiResult;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DnsRecord {
    id: String,
    locked: bool,
    content: IpAddr,
}

impl DnsRecord {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub const fn locked(&self) -> bool {
        self.locked
    }

    pub const fn content(&self) -> IpAddr {
        self.content
    }
}

#[cfg(test)]
impl DnsRecord {
    pub fn new(id: &str, locked: bool, content: IpAddr) -> Self {
        Self { id: id.to_string(), locked, content }
    }
}

impl ApiResult for DnsRecord {}
