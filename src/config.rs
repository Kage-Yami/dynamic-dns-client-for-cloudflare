use argh::FromArgs;

/// A CLI utility to update the A and AAAA DNS records of a domain managed by Cloudflare, from the executing system's
/// current public IP address (written in Rust).
#[derive(FromArgs, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Config {
    /// the name of the DNS zone the domain to update is in
    #[argh(option, short = 'z')]
    zone: String,

    /// the name of the domain to update
    #[argh(option, short = 'd')]
    domain: String,

    /// the API key with permissions to query and update the DNS record
    #[argh(option, short = 'a')]
    api_token: String,

    /// only update the A record (IPv4)
    #[argh(switch, short = '4')]
    only_v4: bool,

    /// only update the AAAA record (IPv6)
    #[argh(switch, short = '6')]
    only_v6: bool,
}

impl Config {
    pub fn zone(&self) -> &str {
        &self.zone
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn api_token(&self) -> &str {
        &self.api_token
    }

    pub fn only_v4(&self) -> bool {
        self.only_v4
    }

    pub fn only_v6(&self) -> bool {
        self.only_v6
    }
}
