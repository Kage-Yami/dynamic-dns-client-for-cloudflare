// Enable all clippy lints and enforce, and opt out of individual lints
#![cfg_attr(feature = "cargo-clippy", warn(clippy::cargo, clippy::pedantic, clippy::nursery))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::default_trait_access, clippy::module_name_repetitions, clippy::must_use_candidate)
)]
//
// Force certain lints to be errors
#![deny(unused_must_use)]
//
#![doc(html_root_url = "https://docs.rs/dynamic-dns-client-for-cloudflare/0.1.5")]

//! # Dynamic DNS Client for Cloudflare® <!-- omit in toc -->
//!
//! - [Overview](#overview)
//!   - [Versioning](#versioning)
//!   - [Repository information](#repository-information)
//! - [Usage](#usage)
//!   - [Once-off update](#once-off-update)
//!   - [Recurring](#recurring)
//!     - [Windows](#windows)
//!     - [Linux - `systemd`](#linux---systemd)
//!   - [Full help extract](#full-help-extract)
//! - [Attributions](#attributions)
//!
//! ## Overview
//!
//! **_This tool has been developed by an unaffiliated third-party, and is not endorsed or supported by Cloudflare._**
//!
//! A CLI utility to update the A and AAAA DNS records of a domain managed by Cloudflare, from the executing system's current public IP address (written in Rust).
//!
//! Please note that only the `windows-x86_64` build gets realistically tested; the tool is built for other platforms "because it can be". Feel free to open issues about them so they're logged, but don't expect much to come out of it.
//!
//! [![Crates.io version](https://img.shields.io/crates/v/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://docs.rs/dynamic-dns-client-for-cloudflare/latest/dynamic-dns-client-for-cloudflare/)
//! [![Crates.io downloads](https://img.shields.io/crates/d/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://crates.io/crates/dynamic-dns-client-for-cloudflare)
//! [![Gitlab pipeline status](https://img.shields.io/gitlab/pipeline/Kage-Yami/dynamic-dns-client-for-cloudflare/main?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare/pipelines/main/latest)
//! [![Gitlab code coverage](https://img.shields.io/gitlab/coverage/Kage-Yami/dynamic-dns-client-for-cloudflare/main?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare)
//! [![Lines of code](https://img.shields.io/tokei/lines/gitlab/Kage-Yami/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare)
//! [![Dependents](https://img.shields.io/librariesio/dependent-repos/cargo/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://libraries.io/cargo/dynamic-dns-client-for-cloudflare)
//! [![License](https://img.shields.io/crates/l/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare/-/blob/main/LICENSE)
//!
//! ### Versioning
//!
//! This project follows [Semantic Versioning principals](https://semver.org/) starting with `1.0.0`.
//!
//! ### Repository information
//!
//! This repository is located on [GitLab.com](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare).
//!
//! There is a [mirror on GitHub](https://github.com/Kage-Yami/dynamic-dns-client-for-cloudflare), but this is not used for development; it is only used for building the tool for each platform. Any issues should be opened on the original GitLab repository.
//!
//! ## Usage
//!
//! **It is _strongly_ recommended that a specialised API token is used only for this. This will reduce the scope of any damage if it were to leak, and reduce the impact when cycling the token.**
//!
//! The permissions required are:
//! - `#zone:read`
//! - `#dns_records:read`
//! - `#dns_records:edit`
//!
//! ### Once-off update
//!
//! To initiate a DNS record update, simply execute the utility like so:
//!
//! Windows:
//!
//! ```powershell
//! ./ddns-for-cloudflare.exe --zone "$ZoneName" --domain "$DomainName" --api-token "$ApiToken"
//! ```
//!
//! Linux:
//!
//! ```sh
//! ./ddns-for-cloudflare --zone "$zone_name" --domain "$domain_name" --api-token "$api_token"
//! ```
//!
//! To only update the A or AAAA record, additionally pass in the `--only-v4` or `--only-v6` switches, respectively.
//!
//! ### Recurring
//!
//! Note that Cloudflare applies a rate limit of 1,200 requests per 5 minutes; this utility makes a total of 5 API calls per execution. For comparison, running the utility every second for 5 minutes would theoretically result in 1,500 requests.
//!
//! #### Windows
//!
//! To execute the utility on a recurring basis in Windows, simply add a scheduled task; a suggested trigger is "on a *daily* schedule" and "repeat task every *1 hour* for a duration of *1 day*".
//!
//! You'll probably also want to log the output, setting the scheduled task to the following command will accomplish this:
//!
//! ```powershell
//! powershell.exe -NonInteractive -Command "./ddns-for-cloudflare.exe --zone '$ZoneName' --domain '$DomainName' --api-token '$ApiToken' *> $LogPath/$((Get-Date).ToString('yyyy-MM-dd HH-mm-ss')).log"
//! ```
//!
//! For convenience, the following PowerShell script can add this scheduled task for you; save it, replace the variables within `$Action` as needed, and then run it with admin rights:
//!
//! ```powershell
//! $Action = New-ScheduledTaskAction -Execute "Powershell.exe" `
//!     -Argument "-NonInteractive -Command `"$ExecutablePath\ddns-for-cloudflare.exe --zone '$ZoneName' --domain '$DomainName' --api-token '$ApiToken' *> $LogPath\`$((Get-Date).ToString('yyyy-MM-dd HH-mm-ss')).log`""
//!
//! $Trigger = New-ScheduledTaskTrigger -Daily -At 9am
//! $TriggerRepeat = New-ScheduledTaskTrigger -Once -At 9am `
//!     -RepetitionInterval $(New-TimeSpan -Hours 1) `
//!     -RepetitionDuration $(New-Timespan -Days 1)
//! $Trigger.Repetition = $TriggerRepeat.Repetition
//!
//! Register-ScheduledTask -Action $Action -Trigger $Trigger -TaskName "Dynamic DNS Client for Cloudflare" -TaskPath "Custom"
//! ```
//!
//! #### Linux - `systemd`
//!
//! With the following `systemd` units, you can execute the utility on a recurring basis in Linux:
//!
//! ```ini
//! [Unit]
//! Description=Dynamic DNS Client for Cloudflare
//! After=network.target
//!
//! [Service]
//! Type=oneshot
//! ExecStart=$executable_path/ddns-for-cloudflare --zone "$zone_name" --domain "$domain_name" --api-token "$api_token"
//!
//! [Install]
//! WantedBy=multi-user.target
//! ```
//!
//! Save the above to `~/.config/systemd/user/ddns-for-cloudflare.service` and update the placeholders as needed.
//!
//! ```ini
//! [Unit]
//! Description=Dynamic DNS Client for Cloudflare - Timer
//!
//! [Timer]
//! OnBootSec=30s
//! OnUnitActiveSec=30m
//!
//! [Install]
//! WantedBy=timers.target
//! ```
//!
//! Save the above to `~/.config/systemd/user/ddns-for-cloudflare.timer`, and then run the following to enable it:
//!
//! ```sh
//! systemctl --user daemon-reload
//! systemctl --user enable --now ddns-for-cloudflare.timer
//! ```
//!
//! ### Full help extract
//!
//! ```
//! Usage: ddns-for-cloudflare.exe -z <zone> -d <domain> -a <api-token> [-4] [-6]
//!
//! A CLI utility to update the A and AAAA DNS records of a domain managed by Cloudflare, from the executing system's current public IP address (written in Rust).
//!
//! Options:
//!   -z, --zone        the name of the DNS zone the domain to update is in
//!   -d, --domain      the name of the domain to update
//!   -a, --api-token   the API key with permissions to query and update the DNS
//!                     record
//!   -4, --only-v4     only update the A record (IPv4)
//!   -6, --only-v6     only update the AAAA record (IPv6)
//!   --help            display usage information
//! ```
//!
//! ## Attributions
//!
//! _Cloudflare is a registered trademark of Cloudflare, Inc._

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
