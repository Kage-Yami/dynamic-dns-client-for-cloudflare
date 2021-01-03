// Enable all clippy lints and enforce, and opt out of individual lints
#![cfg_attr(feature = "cargo-clippy", warn(clippy::cargo, clippy::pedantic, clippy::nursery))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::default_trait_access, clippy::must_use_candidate))]
//
// Force certain lints to be errors
#![deny(unused_must_use)]
//
#![doc(html_root_url = "https://docs.rs/dynamic-dns-client-for-cloudflare/0.1.0")]

//! # Usage
//!
//! **It is _strongly_ recommended that a specialised API token is used only for this. This will reduce the scope of any damage if it were to leak, and reduce the impact when cycling the token.**
//!
//! The permissions required are:
//! - `#zone:read`
//! - `#dns_records:read`
//! - `#dns_records:edit`
//!
//! ## Once-off update
//!
//! To initiate a DNS record update, simply execute the utility like so:
//!
//! Windows:
//!
//! ```powershell
//! ./ddns-for-cloudflare.exe --zone $ZoneName --domain $DomainName --api-token $ApiToken
//! ```
//!
//! Linux:
//!
//! ```sh
//! ./ddns-for-cloudflare --zone $zone_name --domain $domain_name --api-token $api_token
//! ```
//!
//! To only update the A or AAAA record, additionally pass in the `--only-v4` or `--only-v6` switches, respectively.
//!
//! ## Recurring - Windows
//!
//! _To be documented..._
//!
//! ## Recurring - Linux (`systemd`)
//!
//! _To be documented..._

use crate::api::cloudflare::DnsRecordType;
use anyhow::Context;
use config::Config;
use std::net::IpAddr;

#[doc(hidden)]
mod api;

#[doc(hidden)]
mod config;

#[doc(hidden)]
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
        let ip = api::ip::Client::new().v4().context("failed to fetch IPv4 address")?;

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
        let ip = api::ip::Client::new().v6().context("failed to fetch IPv6 address")?;

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
