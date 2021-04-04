#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DnsRecordType {
    A,
    #[allow(clippy::upper_case_acronyms)]
    AAAA,
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::AAAA => write!(f, "AAAA"),
        }
    }
}
