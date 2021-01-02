// Enable all clippy lints and enforce, and opt out of individual lints
#![cfg_attr(feature = "cargo-clippy", warn(clippy::cargo, clippy::pedantic, clippy::nursery))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::must_use_candidate))]
//
// Force certain lints to be errors
#![deny(unused_must_use)]
//
#![doc(html_root_url = "https://docs.rs/dataweave/0.1.0")]

fn main() -> anyhow::Result<()> {
    Ok(())
}
