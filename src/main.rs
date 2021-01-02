// Enable all clippy lints and enforce, and opt out of individual lints
#![cfg_attr(feature = "cargo-clippy", warn(clippy::cargo, clippy::pedantic, clippy::nursery))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::default_trait_access, clippy::must_use_candidate))]
//
// Force certain lints to be errors
#![deny(unused_must_use)]
//
#![doc(html_root_url = "https://docs.rs/dataweave/0.1.0")]

use crate::api::cloudflare::DnsRecordType;
use anyhow::Context;
use config::Config;
use std::net::IpAddr;

mod api;
mod config;

fn main() -> anyhow::Result<()> {
    let config: Config = argh::from_env();

    if config.only_v4() && config.only_v6() {
        anyhow::bail!("--only-v4 and --only-v6 are exclusive arguments; pick one or neither");
    }

    let cloudflare = api::cloudflare::Client::new(config.api_token());
    let zone = cloudflare.fetch_zone(config.zone()).context("failed to fetch DNS Zone")?;

    let ipv4 = if config.only_v6() {
        None
    } else {
        let record = cloudflare
            .fetch_dns_record(zone.id(), config.domain(), DnsRecordType::A)
            .context("failed to fetch DNS A Record")?;
        let ip = api::ip::v4().context("failed to fetch IPv4 address")?;

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
        let ip = api::ip::v6().context("failed to fetch IPv6 address")?;

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
