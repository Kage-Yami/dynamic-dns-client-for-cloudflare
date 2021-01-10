pub use client::Client;
pub use dns_record_type::DnsRecordType;

mod api_error;
mod api_response;
mod api_result;
mod client;
mod dns_record;
mod dns_record_type;
mod zone;

#[cfg(test)]
pub use client::tests;
