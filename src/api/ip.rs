use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ureq::{Request, Response};

const IPV4: &str = "https://ip4only.me/api/";
const IPV6: &str = "https://ip6only.me/api/";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Ip {
    fetch: fn(Request) -> Response,
}

impl Ip {
    pub fn new() -> Self {
        Self { fetch: Self::get }
    }

    fn get(mut request: Request) -> Response {
        request.call()
    }

    pub fn v4(&self) -> anyhow::Result<Ipv4Addr> {
        let response = (self.fetch)(ureq::get(IPV4));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch IPv4 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse IPv4 response")?;
        let ip = body.split(',').nth(1).context("failed to read IPv4 from body")?;

        Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
    }

    pub fn v6(&self) -> anyhow::Result<Ipv6Addr> {
        let response = (self.fetch)(ureq::get(IPV6));

        if response.status() != 200 {
            anyhow::bail!("failed to fetch Ipv6 from API - {} {}", response.status(), response.status_text());
        }

        let body = response.into_string().context("failed to parse Ipv6 response")?;
        let ip = body.split(',').nth(1).context("failed to read Ipv6 from body")?;

        Ipv6Addr::from_str(ip).context("failed to parse Ipv6 address")
    }
}
