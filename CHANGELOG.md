# Changelog <!-- omit in toc -->

- [1.x.x](#1xx)
  - [1.0.0 - _unreleased_](#100---unreleased)
- [Pre-release versions](#pre-release-versions)
  - [0.1.5 - 10-Jan-2021](#015---10-jan-2021)
  - [0.1.4 - 10-Jan-2021](#014---10-jan-2021)
  - [0.1.3 - 10-Jan-2021](#013---10-jan-2021)
  - [0.1.2 - 10-Jan-2021](#012---10-jan-2021)
  - [0.1.1 - 10-Jan-2021](#011---10-jan-2021)
  - [0.1.0 - 10-Jan-2021](#010---10-jan-2021)

## 1.x.x

### 1.0.0 - _unreleased_

- Initial "stable" release:
  - Builds available for Windows (x86_64, i686), Linux (aarch64, x86_64, i686, armv7hf, armhf), and macOS (x86_64)
  - Published to [crates.io](https://crates.io/crates/dynamic-dns-client-for-cloudflare)

## Pre-release versions

### 0.1.5 - 10-Jan-2021

- Artifact script fixes:
  - Input filename for `curl`
  - Raw output for `jq` strings
  - Correct `$CI_COMMIT_REF` → `$CI_COMMIT_REF_SLUG`

### 0.1.4 - 10-Jan-2021

_Note: this release has not been published to crates.io, nor had packages released_

- Fix artifact script to filter to the tag's ref if on a tagged pipeline

### 0.1.3 - 10-Jan-2021

_Note: this release has not been published to crates.io, nor had packages released_

- Fix artifact script to not be restricted to `develop` branch

_Somehow derped, and the above change didn't actually get completed..._

### 0.1.2 - 10-Jan-2021

_Note: this release has not been published to crates.io, nor had packages released_

- Remove Cargo.lock from files to publish (`cargo` reckons it's been changed...)

### 0.1.1 - 10-Jan-2021

_Note: this release has not been published to crates.io, nor had packages released_

- Fix number of artifacts downloaded from GitHub Actions
- Enable publication to crates.io

### 0.1.0 - 10-Jan-2021

_Note: this release has not been published to crates.io, nor had packages released_

- Initial pre-release version
