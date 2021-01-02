use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

const IPV4: &str = "https://ip4only.me/api/";
const IPV6: &str = "https://ip6only.me/api/";

pub fn v4() -> anyhow::Result<Ipv4Addr> {
    let response = ureq::get(IPV4).call().into_string().context("failed to fetch IPv4 from API")?;
    let ip = response.split(',').nth(1).context("failed to read IPv4 from response")?;

    Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
}

pub fn v6() -> anyhow::Result<Ipv6Addr> {
    let response = ureq::get(IPV6).call().into_string().context("failed to fetch IPv6 from API")?;
    let ip = response.split(',').nth(1).context("failed to read IPv6 from response")?;

    Ipv6Addr::from_str(ip).context("failed to parse IPv6 address")
}
