use crate::api::cloudflare::api_response::ApiResponse;
use crate::api::cloudflare::dns_record::DnsRecord;
use crate::api::cloudflare::dns_record_type::DnsRecordType;
use crate::api::cloudflare::zone::Zone;
use anyhow::Context;
use std::net::IpAddr;
use ureq::{json, Request, Response, SerdeValue};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client<'a> {
    api_token: &'a str,
    get_zone: fn(Request) -> Response,
    get_dns_record: fn(Request) -> Response,
    patch_dns_record: fn(Request, SerdeValue) -> Response,
}

impl<'a> Client<'a> {
    pub fn new(api_token: &'a str) -> Self {
        Self { api_token, get_zone: Self::get, get_dns_record: Self::get, patch_dns_record: Self::patch }
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn get(mut request: Request) -> Response {
        request.call()
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn patch(mut request: Request, json: SerdeValue) -> Response {
        request.send_json(json)
    }

    pub fn fetch_zone(&self, zone: &str) -> anyhow::Result<Zone> {
        let mut request = ureq::get("https://api.cloudflare.com/client/v4/zones");
        request
            .query("name", zone)
            .set("content-type", "application/json")
            .set("authorization", &format!("Bearer {}", self.api_token));
        let response = (self.get_zone)(request);

        let body: ApiResponse<Zone> =
            response.into_json_deserialize().context("failed to parse Zones JSON response")?;

        if !body.errors().is_empty() {
            if body.errors().len() > 1 {
                eprintln!("Errors returned from Zones API:");
                for error in body.errors() {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!(
                    "Errors returned from Zones API; first one (see stderr for others): {}",
                    body.errors()[0]
                );
            } else {
                // cannot panic; only runs when body.errors.len() >= 1
                anyhow::bail!("Error returned from Zones API: {}", body.errors()[0]);
            }
        }

        if let Some(mut result) = body.take_result() {
            if result.len() != 1 {
                anyhow::bail!("Unexpected number of Zone results; should be 1: {}", result.len());
            }

            // cannot panic; only runs when result.len() == 1
            Ok(result.swap_remove(0))
        } else {
            anyhow::bail!("Zone results is unexpectedly empty; should be 1 result");
        }
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
        let response = (self.get_dns_record)(request);

        let body: ApiResponse<DnsRecord> =
            response.into_json_deserialize().context("failed to parse DNS Records JSON response")?;

        if !body.errors().is_empty() {
            if body.errors().len() > 1 {
                eprintln!("Errors returned from DNS Records API:");
                for error in body.errors() {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!(
                    "Errors returned from DNS Records API; first one (see stderror for others): {}",
                    body.errors()[0]
                );
            } else {
                // cannot panic; only runs with body.errors.len() >= 1
                anyhow::bail!("Error returned from DNS Records API: {}", body.errors()[0]);
            }
        }

        if let Some(mut result) = body.take_result() {
            if result.len() != 1 {
                anyhow::bail!("Unexpected number of DNS Records results; should be 1: {}", result.len());
            }

            // cannot panic; only runs when result.len() == 1
            Ok(result.swap_remove(0))
        } else {
            anyhow::bail!("DNS Records results is unexpectedly empty; should be 1 result");
        }
    }

    pub fn update_dns_record(&self, zone_id: &str, dns_record_id: &str, ip: IpAddr) -> anyhow::Result<()> {
        let mut request = ureq::patch(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns/records/{identifier}",
            zone_identifier = zone_id,
            identifier = dns_record_id
        ));
        request.set("content-type", "application/json").set("authorization", &format!("Bearer {}", self.api_token));
        let response = (self.patch_dns_record)(request, json!({ "content": ip }));

        let body: ApiResponse<DnsRecord> =
            response.into_json_deserialize().context("failed to parse DNS Records update JSON response")?;

        if !body.errors().is_empty() {
            if body.errors().len() > 1 {
                eprintln!("Errors returned from DNS Records update API:");
                for error in body.errors() {
                    eprintln!("- {}", error);
                }

                // cannot panic; only runs when body.errors.len() > 1
                anyhow::bail!(
                    "Errors returned from DNS Records update API; first one (see stderror for others): {}",
                    body.errors()[0]
                );
            } else {
                // cannot panic; only runs when body.errors.len() >= 1
                anyhow::bail!("Error returned from DNS Records update API: {}", body.errors()[0]);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
impl Client<'_> {
    pub fn set_get_zone(&mut self, get_zone: fn(Request) -> Response) {
        self.get_zone = get_zone;
    }

    pub fn set_get_dns_record(&mut self, get_dns_record: fn(Request) -> Response) {
        self.get_dns_record = get_dns_record;
    }

    pub fn set_patch_dns_record(&mut self, patch_dns_record: fn(Request, SerdeValue) -> Response) {
        self.patch_dns_record = patch_dns_record;
    }
}

#[cfg(test)]
pub mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use anyhow::Context;
    use ureq::{Request, Response, SerdeValue};

    use crate::api::cloudflare;
    use crate::api::cloudflare::dns_record::DnsRecord;
    use crate::api::cloudflare::dns_record_type::DnsRecordType;
    use crate::api::cloudflare::zone::Zone;

    // Not an actual token; taken directly from the API documentation
    const API_TOKEN: &str = "YQSn-xWAQiiEh9qM58wZNnyQS7FUdoqGIUAbrh7T";

    const ZONE_ID: &str = "023e105f4ecef8ad9ca31a8372d0c353";
    const DNS_RECORD_ID: &str = "372e67954025e0ba6aaa6d586b9e0b59";

    #[allow(non_snake_case)]
    fn ZONE() -> Zone {
        Zone::new(ZONE_ID)
    }

    #[allow(non_snake_case)]
    fn DNS_RECORD() -> DnsRecord {
        DnsRecord::new(DNS_RECORD_ID, false, IpAddr::V4(Ipv4Addr::new(198, 51, 100, 4)))
    }

    pub fn mock_zone(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/zone.json"))
    }

    pub fn mock_dns_record(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/dns_record.json"))
    }

    pub fn mock_dns_record_update(_: Request, _: SerdeValue) -> Response {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/dns_record.json"))
    }

    fn mock_failure(_: Request) -> Response {
        Response::new(400, "Bad Request", include_str!("../../../resources/tests/cloudflare/failure.json"))
    }

    #[test]
    fn fetch_zone() -> anyhow::Result<()> {
        let mut client = cloudflare::client::Client::new(API_TOKEN);
        client.get_zone = mock_zone;

        assert_eq!(client.fetch_zone("example.com").context("failed to fetch mock Zone")?, ZONE());

        Ok(())
    }

    #[test]
    fn fetch_dns_record() -> anyhow::Result<()> {
        let mut client = cloudflare::client::Client::new(API_TOKEN);
        client.get_dns_record = mock_dns_record;

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
        let mut client = cloudflare::client::Client::new(API_TOKEN);
        client.patch_dns_record = mock_dns_record_update;

        assert_eq!(
            client
                .update_dns_record(ZONE_ID, DNS_RECORD_ID, IpAddr::V4(Ipv4Addr::LOCALHOST))
                .context("failed to update mock DNS Record")?,
            ()
        );

        Ok(())
    }

    #[test]
    fn failure() -> anyhow::Result<()> {
        let mut client = cloudflare::client::Client::new(API_TOKEN);
        client.get_zone = mock_failure;

        assert_eq!(
            client.fetch_zone("example.com").unwrap_err().to_string(),
            include_str!("../../../resources/tests/cloudflare/failure.txt").trim()
        );

        Ok(())
    }
}
