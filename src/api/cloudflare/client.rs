use crate::api::cloudflare::api_response::{ApiResponseCollection, ApiResponseItem};
use crate::api::cloudflare::dns_record::DnsRecord;
use crate::api::cloudflare::dns_record_type::DnsRecordType;
use crate::api::cloudflare::zone::Zone;
use anyhow::Context;
use std::net::IpAddr;
use ureq::{json, Request, Response, SerdeValue};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client<'a> {
    api_token: &'a str,
    get_zone: fn(Request) -> Result<Response, ureq::Error>,
    get_dns_record: fn(Request) -> Result<Response, ureq::Error>,
    patch_dns_record: fn(Request, SerdeValue) -> Result<Response, ureq::Error>,
}

impl<'a> Client<'a> {
    pub fn new(api_token: &'a str) -> Self {
        Self { api_token, get_zone: Self::get, get_dns_record: Self::get, patch_dns_record: Self::patch }
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn get(request: Request) -> Result<Response, ureq::Error> {
        request.call()
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn patch(request: Request, json: SerdeValue) -> Result<Response, ureq::Error> {
        request.send_json(json)
    }

    pub fn fetch_zone(&self, zone: &str) -> anyhow::Result<Zone> {
        let request = ureq::get("https://api.cloudflare.com/client/v4/zones")
            .query("name", zone)
            .set("content-type", "application/json")
            .set("authorization", &format!("Bearer {}", self.api_token));

        match (self.get_zone)(request) {
            Ok(response) | Err(ureq::Error::Status(_, response)) => {
                let body: ApiResponseCollection<Zone> =
                    response.into_json().context("failed to parse Zones JSON response")?;

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
                    }

                    // cannot panic; only runs when body.errors.len() >= 1
                    anyhow::bail!("Error returned from Zones API: {}", body.errors()[0]);
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
            Err(ureq::Error::Transport(e)) => {
                anyhow::bail!("transport error encountered when fetching Zones from API: {}", e)
            }
        }
    }

    pub fn fetch_dns_record(
        &self,
        zone_id: &str,
        dns_record: &str,
        dns_record_type: DnsRecordType,
    ) -> anyhow::Result<DnsRecord> {
        let request = ureq::get(&format!(
            "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns_records",
            zone_identifier = zone_id
        ))
        .query("name", dns_record)
        .query("type", &dns_record_type.to_string())
        .set("content-type", "application/json")
        .set("authorization", &format!("Bearer {}", self.api_token));

        match (self.get_dns_record)(request) {
            Ok(response) | Err(ureq::Error::Status(_, response)) => {
                let body: ApiResponseCollection<DnsRecord> =
                    response.into_json().context("failed to parse DNS Records JSON response")?;

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
                    }

                    // cannot panic; only runs with body.errors.len() >= 1
                    anyhow::bail!("Error returned from DNS Records API: {}", body.errors()[0]);
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
            Err(ureq::Error::Transport(e)) => {
                anyhow::bail!("transport error encountered when fetching Zones from API: {}", e)
            }
        }
    }

    pub fn update_dns_record(&self, zone_id: &str, dns_record_id: &str, ip: IpAddr) -> anyhow::Result<()> {
        let request = ureq::request(
            "PATCH",
            &format!(
                "https://api.cloudflare.com/client/v4/zones/{zone_identifier}/dns_records/{identifier}",
                zone_identifier = zone_id,
                identifier = dns_record_id
            ),
        )
        .set("content-type", "application/json")
        .set("authorization", &format!("Bearer {}", self.api_token));

        match (self.patch_dns_record)(request, json!({ "content": ip })) {
            Ok(response) | Err(ureq::Error::Status(_, response)) => {
                let body: ApiResponseItem<DnsRecord> =
                    response.into_json().context("failed to parse DNS Records update JSON response")?;

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
                    }
                    // cannot panic; only runs when body.errors.len() >= 1
                    anyhow::bail!("Error returned from DNS Records update API: {}", body.errors()[0]);
                }

                Ok(())
            }
            Err(ureq::Error::Transport(e)) => {
                anyhow::bail!("transport error encountered when fetching Zones from API: {}", e)
            }
        }
    }
}

#[cfg(test)]
impl Client<'_> {
    pub fn set_get_zone(&mut self, get_zone: fn(Request) -> Result<Response, ureq::Error>) {
        self.get_zone = get_zone;
    }

    pub fn set_get_dns_record(&mut self, get_dns_record: fn(Request) -> Result<Response, ureq::Error>) {
        self.get_dns_record = get_dns_record;
    }

    pub fn set_patch_dns_record(&mut self, patch_dns_record: fn(Request, SerdeValue) -> Result<Response, ureq::Error>) {
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

    pub fn mock_zone(_: Request) -> Result<Response, ureq::Error> {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/zone.json"))
    }

    pub fn mock_dns_record(_: Request) -> Result<Response, ureq::Error> {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/dns_record.json"))
    }

    pub fn mock_dns_record_update(_: Request, _: SerdeValue) -> Result<Response, ureq::Error> {
        Response::new(200, "OK", include_str!("../../../resources/tests/cloudflare/dns_record_update.json"))
    }

    fn mock_failure(_: Request) -> Result<Response, ureq::Error> {
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
