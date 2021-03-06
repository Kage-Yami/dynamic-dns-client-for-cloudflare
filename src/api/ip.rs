use anyhow::Context;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use ureq::{Request, Response};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Client {
    fetch_v4: fn(Request) -> Result<Response, ureq::Error>,
    fetch_v6: fn(Request) -> Result<Response, ureq::Error>,
}

impl Client {
    pub fn new() -> Self {
        Self { fetch_v4: Self::get, fetch_v6: Self::get }
    }

    // mocked
    #[cfg(not(tarpaulin_include))]
    fn get(request: Request) -> Result<Response, ureq::Error> {
        request.call()
    }

    pub fn v4(self) -> anyhow::Result<Ipv4Addr> {
        match (self.fetch_v4)(ureq::get("https://api.ipify.org/")) {
            Ok(response) => {
                let body = response.into_string().context("failed to parse IPv4 response")?;
                let ip = body.trim();

                Ipv4Addr::from_str(ip).context("failed to parse IPv4 address")
            }
            Err(ureq::Error::Status(code, _)) => anyhow::bail!("failed to fetch IPv4 from API: {}", code),
            Err(ureq::Error::Transport(e)) => {
                anyhow::bail!("transport error encountered when fetching IPv4 from API: {}", e)
            }
        }
    }

    pub fn v6(self) -> anyhow::Result<Ipv6Addr> {
        match (self.fetch_v6)(ureq::get("https://api6.ipify.org/")) {
            Ok(response) => {
                let body = response.into_string().context("failed to parse IPv6 response")?;
                let ip = body.trim();

                Ipv6Addr::from_str(ip).context("failed to parse IPv6 address")
            }
            Err(ureq::Error::Status(code, _)) => anyhow::bail!("failed to fetch IPv6 from API: {}", code),
            Err(ureq::Error::Transport(e)) => {
                anyhow::bail!("transport error encountered when fetching IPv6 from API: {}", e)
            }
        }
    }
}

#[cfg(test)]
impl Client {
    pub fn set_fetch_v4(&mut self, fetch: fn(Request) -> Result<Response, ureq::Error>) {
        self.fetch_v4 = fetch;
    }

    pub fn set_fetch_v6(&mut self, fetch: fn(Request) -> Result<Response, ureq::Error>) {
        self.fetch_v6 = fetch;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::api;
    use anyhow::Context;
    use std::net::{Ipv4Addr, Ipv6Addr};
    use ureq::{Request, Response};

    pub fn mock_v4(_: Request) -> Result<Response, ureq::Error> {
        Response::new(200, "OK", "127.0.0.1")
    }

    pub fn mock_v6(_: Request) -> Result<Response, ureq::Error> {
        Response::new(200, "OK", "::1")
    }

    #[test]
    fn v4() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch_v4 = mock_v4;

        assert_eq!(client.v4().context("failed to fetch mock IPv4 address")?, Ipv4Addr::LOCALHOST);

        Ok(())
    }

    #[test]
    fn v6() -> anyhow::Result<()> {
        let mut client = api::ip::Client::new();
        client.fetch_v6 = mock_v6;

        assert_eq!(client.v6().context("failed to fetch mock IPv6 address")?, Ipv6Addr::LOCALHOST);

        Ok(())
    }
}
