use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client<'a> {
    api_token: &'a str,
}

impl<'a> Client<'a> {
    pub fn new(api_token: &'a str) -> Self {
        Self { api_token }
    }

    pub fn fetch_zone(&self, zone: &str) -> anyhow::Result<String> {
        let response = ureq::get("https://api.cloudflare.com/client/v4/zones")
            .query("name", zone)
            .set("content-type", "application/json")
            .set("authorization", &format!("Bearer {}", self.api_token))
            .call();
        let mut body: ApiResponse<Zone> =
            response.into_json_deserialize().context("failed to parse Zones JSON response")?;

        if !body.errors.is_empty() {
            if body.errors.len() > 1 {
                eprintln!("Errors returned from Zones API:");
                for error in &body.errors {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!("Errors returned from Zones API; first one (see stderr for others): {}", body.errors[0]);
            } else {
                // cannot panic; only runs when body.errors.len() >= 1
                anyhow::bail!("Error returned from Zones API: {}", body.errors[0])
            }
        }

        if body.result.len() != 1 {
            anyhow::bail!("Unexpected number of Zone results; should be 1: {}", body.result.len());
        }

        // cannot panic; only runs when body.result.len() == 1
        Ok(body.result.swap_remove(0).id)
    }

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct ApiResponse<T: ApiResult> {
    result: Vec<T>,
    errors: Vec<ApiError>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct ApiError {
    code: i128,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl Error for ApiError {}

trait ApiResult {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct Zone {
    id: String,
}

impl ApiResult for Zone {}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct DnsRecord {
    id: String,
    locked: bool,
    content: IpAddr,
}

impl ApiResult for DnsRecord {}
