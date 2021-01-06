use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ureq::{Request, Response};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client {
    fetch_v4: fn(Request) -> Response,
    fetch_v6: fn(Request) -> Response,
}

impl Client {
    pub fn new() -> Self {
        Self { fetch_v4: Self::get, fetch_v6: Self::get }
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn get(mut request: Request) -> Response {
        request.call()
    }

    pub fn v4(self) -> anyhow::Result<Ipv4Addr> {
        let response = (self.fetch_v4)(ureq::get("https://ip4only.me/api/"));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch IPv4 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse IPv4 response")?;
        let ip = body.split(',').nth(1).context("failed to read IPv4 from body")?;

        Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
    }

    pub fn v6(self) -> anyhow::Result<Ipv6Addr> {
        let response = (self.fetch_v6)(ureq::get("https://ip6only.me/api/"));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch IPv6 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse IPv6 response")?;
        let ip = body.split(',').nth(1).context("failed to read IPv6 from body")?;

        Ipv6Addr::from_str(ip).context("failed to parse IPv6 address")
    }
}

#[cfg(test)]
impl Client {
    pub fn set_fetch_v4(&mut self, fetch: fn(Request) -> Response) {
        self.fetch_v4 = fetch;
    }

    pub fn set_fetch_v6(&mut self, fetch: fn(Request) -> Response) {
        self.fetch_v6 = fetch;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::api;
    use anyhow::Context;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use ureq::{Request, Response};

    pub fn mock_v4(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/ip/v4.csv"))
    }

    pub fn mock_v6(_: Request) -> Response {
        Response::new(200, "OK", include_str!("../../resources/tests/ip/v6.csv"))
    }

    #[test]
    fn v4() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch_v4 = mock_v4;

        assert_eq!(client.v4().context("failed to fetch mock IPv4 address")?, Ipv4Addr::LOCALHOST);

        Ok(())
    }

    #[test]
    fn v6() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch_v6 = mock_v6;

        assert_eq!(client.v6().context("failed to fetch mock IPv6 address")?, Ipv6Addr::LOCALHOST);

        Ok(())
    }
}
