use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use ureq::{json, Request, Response, SerdeValue};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client<'a> {
    api_token: &'a str,
    fetch: fn(Request) -> Response,
    update: fn(Request, SerdeValue) -> Response,
}

impl<'a> Client<'a> {
    pub fn new(api_token: &'a str) -> Self {
        Self { api_token, fetch: Self::get, update: Self::patch }
    }

    fn get(mut request: Request) -> Response {
        request.call()
    }

    fn patch(mut request: Request, json: SerdeValue) -> Response {
        request.send_json(json)
    }

    pub fn fetch_zone(&self, zone: &str) -> anyhow::Result<Zone> {
        let mut request = ureq::get("https://api.cloudflare.com/client/v4/zones");
        request
            .query("name", zone)
            .set("content-type", "application/json")
            .set("authorization", &format!("Bearer {}", self.api_token));
        let response = (self.fetch)(request);

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
        let mut request = ureq::get(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns/records",
            zone_identifier = zone_id
        ));
        request
            .query("name", dns_record)
            .query("type", &dns_record_type.to_string())
            .set("content-type", "application/json")
            .set("authorization", &format!("Bearer {}", self.api_token));
        let response = (self.fetch)(request);

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
        let mut request = ureq::patch(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns/records/{identifier}",
            zone_identifier = zone_id,
            identifier = dns_record_id
        ));
        request.set("content-type", "application/json").set("authorization", &format!("Bearer {}", self.api_token));
        let response = (self.update)(request, json!({ "content": ip }));

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

#[cfg(test)]
mod tests {
    use crate::api;
    use crate::api::cloudflare::{DnsRecord, DnsRecordType, Zone};
    use anyhow::Context;
    use std::net::{IpAddr, Ipv4Addr};
    use ureq::{Request, Response, SerdeValue};

    // Not an actual token; taken directly from the API documentation
    const API_TOKEN: &str = "YQSn-xWAQiiEh9qM58wZNnyQS7FUdoqGIUAbrh7T";

    const ZONE_ID: &str = "023e105f4ecef8ad9ca31a8372d0c353";
    const DNS_RECORD_ID: &str = "372e67954025e0ba6aaa6d586b9e0b59";

    #[allow(non_snake_case)]
    fn ZONE() -> Zone {
        Zone { id: ZONE_ID.to_string() }
    }

    #[allow(non_snake_case)]
    fn DNS_RECORD() -> DnsRecord {
        DnsRecord { id: DNS_RECORD_ID.to_string(), locked: false, content: IpAddr::V4(Ipv4Addr::new(198, 51, 100, 4)) }
    }

    fn mock_zone(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/cloudflare/zone.json"))
    }

    fn mock_dns_record(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/cloudflare/dns_record.json"))
    }

    fn mock_dns_record_update(_: Request, _: SerdeValue) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/cloudflare/dns_record.json"))
    }

    #[test]
    fn fetch_zone() -> anyhow::Result<()> {
        let mut client = api::cloudflare::Client::new(API_TOKEN);
        client.fetch = mock_zone;

        assert_eq!(client.fetch_zone("example.com").context("failed to fetch mock Zone")?, ZONE());

        Ok(())
    }

    #[test]
    fn fetch_dns_record() -> anyhow::Result<()> {
        let mut client = api::cloudflare::Client::new(API_TOKEN);
        client.fetch = mock_dns_record;

        assert_eq!(
            client
                .fetch_dns_record(ZONE_ID, "example.com", DnsRecordType::A)
                .context("failed to fetch mock DNS Record")?,
            DNS_RECORD()
        );

        Ok(())
    }

    #[test]
    fn update_dns_record() -> anyhow::Result<()> {
        let mut client = api::cloudflare::Client::new(API_TOKEN);
        client.update = mock_dns_record_update;

        assert_eq!(
            client
                .update_dns_record(ZONE_ID, DNS_RECORD_ID, IpAddr::V4(Ipv4Addr::LOCALHOST))
                .context("failed to update mock DNS Record")?,
            ()
        );

        Ok(())
    }
}
