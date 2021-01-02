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

    /// the email address of the domain's owner
    #[argh(option, short = 'e')]
    email: String,

    /// the API key with permissions to query and update the DNS record
    #[argh(option, short = 'a')]
    api_key: String,
}
