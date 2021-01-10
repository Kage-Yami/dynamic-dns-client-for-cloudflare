# Dynamic DNS Client for Cloudflare® <!-- omit in toc -->

- [Overview](#overview)
  - [Versioning](#versioning)
  - [Repository information](#repository-information)
- [Usage](#usage)
  - [Once-off update](#once-off-update)
  - [Recurring](#recurring)
    - [Windows](#windows)
    - [Linux - `systemd`](#linux---systemd)
  - [Full help extract](#full-help-extract)
- [Attributions](#attributions)

## Overview

**_This tool has been developed by an unaffiliated third-party, and is not endorsed or supported by Cloudflare._**

A CLI utility to update the A and AAAA DNS records of a domain managed by Cloudflare, from the executing system's current public IP address (written in Rust).

Please note that only the `windows-x86_64` build gets realistically tested; the tool is built for other platforms "because it can be". Feel free to open issues about them so they're logged, but don't expect much to come out of it.

[![Crates.io version](https://img.shields.io/crates/v/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://docs.rs/dynamic-dns-client-for-cloudflare/latest/dynamic-dns-client-for-cloudflare/)
[![Crates.io downloads](https://img.shields.io/crates/d/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://crates.io/crates/dynamic-dns-client-for-cloudflare)
[![Gitlab pipeline status](https://img.shields.io/gitlab/pipeline/Kage-Yami/dynamic-dns-client-for-cloudflare/main?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare/pipelines/main/latest)
[![Gitlab code coverage](https://img.shields.io/gitlab/coverage/Kage-Yami/dynamic-dns-client-for-cloudflare/main?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare)
[![Lines of code](https://img.shields.io/tokei/lines/gitlab/Kage-Yami/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare)
[![Dependents](https://img.shields.io/librariesio/dependent-repos/cargo/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://libraries.io/cargo/dynamic-dns-client-for-cloudflare)
[![License](https://img.shields.io/crates/l/dynamic-dns-client-for-cloudflare?style=for-the-badge)](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare/-/blob/main/LICENSE)

### Versioning

This project follows [Semantic Versioning principals](https://semver.org/) starting with `1.0.0`.

### Repository information

This repository is located on [GitLab.com](https://gitlab.com/Kage-Yami/dynamic-dns-client-for-cloudflare).

There is a [mirror on GitHub](https://github.com/Kage-Yami/dynamic-dns-client-for-cloudflare), but this is not used for development; it is only used for building the tool for each platform. Any issues should be opened on the original GitLab repository.

## Usage

**It is _strongly_ recommended that a specialised API token is used only for this. This will reduce the scope of any damage if it were to leak, and reduce the impact when cycling the token.**

The permissions required are:
- `#zone:read`
- `#dns_records:read`
- `#dns_records:edit`

### Once-off update

To initiate a DNS record update, simply execute the utility like so:

Windows:

```powershell
./ddns-for-cloudflare.exe --zone "$ZoneName" --domain "$DomainName" --api-token "$ApiToken"
```

Linux:

```sh
./ddns-for-cloudflare --zone "$zone_name" --domain "$domain_name" --api-token "$api_token"
```

To only update the A or AAAA record, additionally pass in the `--only-v4` or `--only-v6` switches, respectively.

### Recurring

Note that Cloudflare applies a rate limit of 1,200 requests per 5 minutes; this utility makes a total of 5 API calls per execution. For comparison, running the utility every second for 5 minutes would theoretically result in 1,500 requests.

#### Windows

To execute the utility on a recurring basis in Windows, simply add a scheduled task; a suggested trigger is "on a *daily* schedule" and "repeat task every *1 hour* for a duration of *1 day*".

You'll probably also want to log the output, setting the scheduled task to the following command will accomplish this:

```powershell
powershell.exe -NonInteractive -Command "./ddns-for-cloudflare.exe --zone '$ZoneName' --domain '$DomainName' --api-token '$ApiToken' *> $LogPath/$((Get-Date).ToString('yyyy-MM-dd HH-mm-ss')).log"
```

For convenience, the following PowerShell script can add this scheduled task for you; save it, replace the variables within `$Action` as needed, and then run it with admin rights:

```powershell
$Action = New-ScheduledTaskAction -Execute "Powershell.exe" `
    -Argument "-NonInteractive -Command `"$ExecutablePath\ddns-for-cloudflare.exe --zone '$ZoneName' --domain '$DomainName' --api-token '$ApiToken' *> $LogPath\`$((Get-Date).ToString('yyyy-MM-dd HH-mm-ss')).log`""

$Trigger = New-ScheduledTaskTrigger -Daily -At 9am
$TriggerRepeat = New-ScheduledTaskTrigger -Once -At 9am `
    -RepetitionInterval $(New-TimeSpan -Hours 1) `
    -RepetitionDuration $(New-Timespan -Days 1)
$Trigger.Repetition = $TriggerRepeat.Repetition

Register-ScheduledTask -Action $Action -Trigger $Trigger -TaskName "Dynamic DNS Client for Cloudflare" -TaskPath "Custom"
```

#### Linux - `systemd`

With the following `systemd` units, you can execute the utility on a recurring basis in Linux:

```ini
[Unit]
Description=Dynamic DNS Client for Cloudflare
After=network.target

[Service]
Type=oneshot
ExecStart=$executable_path/ddns-for-cloudflare --zone "$zone_name" --domain "$domain_name" --api-token "$api_token"

[Install]
WantedBy=multi-user.target
```

Save the above to `~/.config/systemd/user/ddns-for-cloudflare.service` and update the placeholders as needed.

```ini
[Unit]
Description=Dynamic DNS Client for Cloudflare - Timer

[Timer]
OnBootSec=30s
OnUnitActiveSec=30m

[Install]
WantedBy=timers.target
```

Save the above to `~/.config/systemd/user/ddns-for-cloudflare.timer`, and then run the following to enable it:

```sh
systemctl --user daemon-reload
systemctl --user enable --now ddns-for-cloudflare.timer
```

### Full help extract

```
Usage: ddns-for-cloudflare.exe -z <zone> -d <domain> -a <api-token> [-4] [-6]

A CLI utility to update the A and AAAA DNS records of a domain managed by Cloudflare, from the executing system's current public IP address (written in Rust).

Options:
  -z, --zone        the name of the DNS zone the domain to update is in
  -d, --domain      the name of the domain to update
  -a, --api-token   the API key with permissions to query and update the DNS
                    record
  -4, --only-v4     only update the A record (IPv4)
  -6, --only-v6     only update the AAAA record (IPv6)
  --help            display usage information
```

## Attributions

_Cloudflare is a registered trademark of Cloudflare, Inc._
