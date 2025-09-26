# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/ariel-os/sbd/releases/tag/v0.1.0) - 2025-09-26

### Added

- *(ariel)* introduce StringOrWorkspace
- *(ariel)* add `--overwrite` flag
- *(ariel)* add header comment with yamllint ignore to laze file
- update Crate manifest edition/rust-version handling
- initial RIOT support

### Fixed

- *(ariel)* add `// @generated` to generated rust files
- *(ariel)* allow unused variables / imports`

### Other

- factor out file writing
- update Cargo.toml
- rename to sbd-gen
- fix clippy pedantic
- run clippy
- fix some lints
- add Rust build workflow
- add initial dependabot config
- *(ariel)* introduce `--mode`
- soc -> chip
- *(ariel)* create per-board .rs files
- introduce generate-ariel subcommand
- add README.md
- initial commit
