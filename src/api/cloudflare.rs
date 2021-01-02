use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use ureq::json;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client<'a> {
    api_token: &'a str,
}

impl<'a> Client<'a> {
    pub const fn new(api_token: &'a str) -> Self {
        Self { api_token }
    }

    pub fn fetch_zone(&self, zone: &str) -> anyhow::Result<Zone> {
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
                anyhow::bail!("Error returned from Zones API: {}", body.errors[0]);
            }
        }

        if body.result.len() != 1 {
            anyhow::bail!("Unexpected number of Zone results; should be 1: {}", body.result.len());
        }

        // cannot panic; only runs when body.result.len() == 1
        Ok(body.result.swap_remove(0))
    }

    pub fn fetch_dns_record(
        &self,
        zone_id: &str,
        dns_record: &str,
        dns_record_type: DnsRecordType,
    ) -> anyhow::Result<DnsRecord> {
        let response = ureq::get(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns/records",
            zone_identifier = zone_id
        ))
        .query("name", dns_record)
        .query("type", &dns_record_type.to_string())
        .set("content-type", "application/json")
        .set("authorization", &format!("Bearer {}", self.api_token))
        .call();

        let mut body: ApiResponse<DnsRecord> =
            response.into_json_deserialize().context("failed to parse DNS Records JSON response")?;

        if !body.errors.is_empty() {
            if body.errors.len() > 1 {
                eprintln!("Errors returned from DNS Records API:");
                for error in &body.errors {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!(
                    "Errors returned from DNS Records API; first one (see stderror for others): {}",
                    body.errors[0]
                );
            } else {
                // cannot panic; only runs with body.errors.len() >= 1
                anyhow::bail!("Error returned from DNS Records API: {}", body.errors[0]);
            }
        }

        if body.result.len() != 1 {
            anyhow::bail!("Unexpected number of DNS Records results; should be 1: {}", body.result.len());
        }

        // cannot panic; only runs when body.result.len() == 1
        Ok(body.result.swap_remove(0))
    }

    pub fn update_dns_record(&self, zone_id: &str, dns_record_id: &str, ip: IpAddr) -> anyhow::Result<()> {
        let response = ureq::patch(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns/records/{identifier}",
            zone_identifier = zone_id,
            identifier = dns_record_id
        ))
        .set("content-type", "application/json")
        .set("authorization", &format!("Bearer {}", self.api_token))
        .send_json(json!({ "content": ip }));

        let body: ApiResponse<DnsRecord> =
            response.into_json_deserialize().context("failed to parse DNS Records update JSON response")?;

        if !body.errors.is_empty() {
            if body.errors.len() > 1 {
                eprintln!("Errors returned from DNS Records update API:");
                for error in &body.errors {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!(
                    "Errors returned from DNS Records update API; first one (see stderror for others): {}",
                    body.errors[0]
                );
            } else {
                // cannot panic; only runs when body.errors.len() >= 1
                anyhow::bail!("Error returned from DNS Records update API: {}", body.errors[0]);
            }
        }

        Ok(())
    }
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
pub struct Zone {
    id: String,
}

impl Zone {
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl ApiResult for Zone {}

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

impl ApiResult for DnsRecord {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DnsRecordType {
    A,
    AAAA,
}

impl Display for DnsRecordType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::AAAA => write!(f, "AAAA"),
        }
    }
}
