use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

const IPV4: &str = "https://ip4only.me/api/";
const IPV6: &str = "https://ip6only.me/api/";

pub fn v4() -> anyhow::Result<Ipv4Addr> {
    let response = ureq::get(IPV4).call();

    if response.status() != 200 {
        anyhow::bail!("failed to fetch IPv4 from API - {} {}", response.status(), response.status_text());
    }

    let body = response.into_string().context("failed to parse IPv4 response")?;
    let ip = body.split(',').nth(1).context("failed to read IPv4 from body")?;

    Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
}

pub fn v6() -> anyhow::Result<Ipv6Addr> {
    let response = ureq::get(IPV6).call();

    if response.status() != 200 {
        anyhow::bail!("failed to fetch Ipv6 from API - {} {}", response.status(), response.status_text());
    }

    let body = response.into_string().context("failed to parse Ipv6 response")?;
    let ip = body.split(',').nth(1).context("failed to read Ipv6 from body")?;

    Ipv6Addr::from_str(ip).context("failed to parse Ipv6 address")
}
