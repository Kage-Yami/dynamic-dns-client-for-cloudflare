// Enable all clippy lints and enforce, and opt out of individual lints
#![cfg_attr(feature = "cargo-clippy", warn(clippy::cargo, clippy::pedantic, clippy::nursery))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::default_trait_access, clippy::module_name_repetitions, clippy::must_use_candidate)
)]
//
// Force certain lints to be errors
#![deny(unused_must_use)]

use anyhow::Context;
use api::cloudflare;
use api::cloudflare::DnsRecordType;
use api::ip;
use config::Config;
use std::net::IpAddr;

#[doc(hidden)]
mod api;

#[doc(hidden)]
mod config;

// mocked
#[doc(hidden)]
#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let config: Config = argh::from_env();

    if config.only_v4() && config.only_v6() {
        anyhow::bail!("--only-v4 and --only-v6 are exclusive arguments; pick one or neither");
    }

    let cloudflare = cloudflare::Client::new(config.api_token());
    let ip = ip::Client::new();

    update(&config, cloudflare, ip)
}

#[doc(hidden)]
fn update(config: &Config, cloudflare: cloudflare::Client, ip: ip::Client) -> anyhow::Result<()> {
    let zone = cloudflare.fetch_zone(config.zone()).context("failed to fetch DNS Zone")?;

    let ipv4 = if config.only_v6() {
        None
    } else {
        let record = cloudflare
            .fetch_dns_record(zone.id(), config.domain(), DnsRecordType::A)
            .context("failed to fetch DNS A Record")?;
        let ip = ip.v4().context("failed to fetch IPv4 address")?;

        if record.content() == ip {
            println!("A Record already matches desired IPv4; skipping...");
            None
        } else if record.locked() {
            println!("A Record is locked; skipping...");
            None
        } else {
            Some((record, ip))
        }
    };

    let ipv6 = if config.only_v4() {
        None
    } else {
        let record = cloudflare
            .fetch_dns_record(zone.id(), config.domain(), DnsRecordType::AAAA)
            .context("failed to fetch DNS AAAA Record")?;
        let ip = ip.v6().context("failed to fetch IPv6 address")?;

        if record.content() == ip {
            println!("AAAA Record already matches desired IPv6; skipping...");
            None
        } else if record.locked() {
            println!("AAAA Record is locked; skipping...");
            None
        } else {
            Some((record, ip))
        }
    };

    if let Some((record, ip)) = ipv4 {
        cloudflare
            .update_dns_record(zone.id(), record.id(), IpAddr::V4(ip))
            .context("failed to update DNS A Record")?;

        println!("A Record updated to: {}", ip);
    }

    if let Some((record, ip)) = ipv6 {
        cloudflare
            .update_dns_record(zone.id(), record.id(), IpAddr::V6(ip))
            .context("failed to update DNS AAAA Record")?;

        println!("AAAA Record updated to: {}", ip);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::cloudflare::tests::{mock_dns_record, mock_dns_record_update, mock_zone};
    use crate::api::ip::tests::{mock_v4, mock_v6};
    use crate::api::{cloudflare, ip};
    use crate::config::Config;
    use crate::update;

    // Not an actual token; taken directly from the API documentation
    const API_TOKEN: &str = "YQSn-xWAQiiEh9qM58wZNnyQS7FUdoqGIUAbrh7T";

    #[test]
    fn update_mocked() -> anyhow::Result<()> {
        let config = Config::new("example.com", "example.com", API_TOKEN, false, false);

        let mut cloudflare = cloudflare::Client::new(config.api_token());
        let mut ip = ip::Client::new();

        cloudflare.set_get_zone(mock_zone);
        cloudflare.set_get_dns_record(mock_dns_record);
        cloudflare.set_patch_dns_record(mock_dns_record_update);

        ip.set_fetch_v4(mock_v4);
        ip.set_fetch_v6(mock_v6);

        update(&config, cloudflare, ip)
    }

    #[test]
    fn update_mocked_v4_only() -> anyhow::Result<()> {
        let config = Config::new("example.com", "example.com", API_TOKEN, true, false);

        let mut cloudflare = cloudflare::Client::new(config.api_token());
        let mut ip = ip::Client::new();

        cloudflare.set_get_zone(mock_zone);
        cloudflare.set_get_dns_record(mock_dns_record);
        cloudflare.set_patch_dns_record(mock_dns_record_update);

        ip.set_fetch_v4(mock_v4);
        ip.set_fetch_v6(mock_v6);

        update(&config, cloudflare, ip)
    }

    #[test]
    fn update_mocked_v6_only() -> anyhow::Result<()> {
        let config = Config::new("example.com", "example.com", API_TOKEN, false, true);

        let mut cloudflare = cloudflare::Client::new(config.api_token());
        let mut ip = ip::Client::new();

        cloudflare.set_get_zone(mock_zone);
        cloudflare.set_get_dns_record(mock_dns_record);
        cloudflare.set_patch_dns_record(mock_dns_record_update);

        ip.set_fetch_v4(mock_v4);
        ip.set_fetch_v6(mock_v6);

        update(&config, cloudflare, ip)
    }
}
