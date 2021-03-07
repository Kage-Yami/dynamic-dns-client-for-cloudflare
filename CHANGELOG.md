# Changelog <!-- omit in toc -->

- [1.x.x](#1xx)
  - [1.1.0 - 07-Mar-2021](#110---07-mar-2021)
  - [1.0.0 - 20-Jan-2021](#100---20-jan-2021)
- [Pre-release versions](#pre-release-versions)
  - [0.1.8 - 20-Jan-2021](#018---20-jan-2021)
  - [0.1.7 - 20-Jan-2021](#017---20-jan-2021)
  - [0.1.6 - 10-Jan-2021](#016---10-jan-2021)
  - [0.1.5 - 10-Jan-2021](#015---10-jan-2021)
  - [0.1.4 - 10-Jan-2021](#014---10-jan-2021)
  - [0.1.3 - 10-Jan-2021](#013---10-jan-2021)
  - [0.1.2 - 10-Jan-2021](#012---10-jan-2021)
  - [0.1.1 - 10-Jan-2021](#011---10-jan-2021)
  - [0.1.0 - 10-Jan-2021](#010---10-jan-2021)

## 1.x.x

### 1.1.0 - 07-Mar-2021

- Bump `serde` to `1.0.124`

### 1.0.0 - 20-Jan-2021

- Initial "stable" release; builds available for:
  - Windows (x86_64, i686)
  - Linux (aarch64, x86_64, i686, armv7hf, armhf)
  - macOS (x86_64)

## Pre-release versions

### 0.1.8 - 20-Jan-2021

- Make `curl` follow `location` header for artifact download

### 0.1.7 - 20-Jan-2021

_Note: this release has not had packaged released_

- Replace `tar` with `unzip`
- Update dependencies
- Update Docker image versions

### 0.1.6 - 10-Jan-2021

_Note: this release has not had packages released_

- Fix `tar` to extract using `gzip`

### 0.1.5 - 10-Jan-2021

_Note: this release has not had packages released_

- Artifact script fixes:
  - Input filename for `curl`
  - Raw output for `jq` strings
  - Correct `$CI_COMMIT_REF` → `$CI_COMMIT_REF_SLUG`

### 0.1.4 - 10-Jan-2021

_Note: this release has not had packages released_

- Fix artifact script to filter to the tag's ref if on a tagged pipeline

### 0.1.3 - 10-Jan-2021

_Note: this release has not had packages released_

- Fix artifact script to not be restricted to `develop` branch

_Somehow derped, and the above change didn't actually get completed..._

### 0.1.2 - 10-Jan-2021

_Note: this release has not had packages released_

- Remove Cargo.lock from files to publish (`cargo` reckons it's been changed...)

### 0.1.1 - 10-Jan-2021

_Note: this release has not had packages released_

- Fix number of artifacts downloaded from GitHub Actions
- Enable publication to crates.io

### 0.1.0 - 10-Jan-2021

_Note: this release has not had packages released_

- Initial pre-release version
