use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ureq::{Request, Response};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client {
    fetch: fn(Request) -> Response,
}

impl Client {
    pub fn new() -> Self {
        Self { fetch: Self::get }
    }

    fn get(mut request: Request) -> Response {
        request.call()
    }

    pub fn v4(&self) -> anyhow::Result<Ipv4Addr> {
        let response = (self.fetch)(ureq::get("https://ip4only.me/api/"));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch IPv4 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse IPv4 response")?;
        let ip = body.split(',').nth(1).context("failed to read IPv4 from body")?;

        Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
    }

    pub fn v6(&self) -> anyhow::Result<Ipv6Addr> {
        let response = (self.fetch)(ureq::get("https://ip6only.me/api/"));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch Ipv6 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse Ipv6 response")?;
        let ip = body.split(',').nth(1).context("failed to read Ipv6 from body")?;

        Ipv6Addr::from_str(ip).context("failed to parse Ipv6 address")
    }
}

#[cfg(test)]
mod tests {
    use crate::api;
    use anyhow::Context;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use ureq::{Request, Response};

    fn get_mock_v4(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/ip/v4.csv"))
    }

    fn get_mock_v6(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/ip/v6.csv"))
    }

    #[test]
    fn v4() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch = get_mock_v4;

        assert_eq!(client.v4().context("failed to get mock IPv4 address")?, Ipv4Addr::LOCALHOST);

        Ok(())
    }

    #[test]
    fn v6() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch = get_mock_v6;

        assert_eq!(client.v6().context("failed to get mock IPv6 address")?, Ipv6Addr::LOCALHOST);

        Ok(())
    }
}
