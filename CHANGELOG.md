# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.11](https://github.com/ariel-os/sbd/compare/v0.1.10...v0.1.11) - 2025-12-05

### Other

- Generated code: condition

## [0.1.10](https://github.com/ariel-os/sbd/compare/v0.1.9...v0.1.10) - 2025-12-05

### Other

- *(deps)* bump yaml-hash from 0.5.1 to 0.6.1
- *(deps)* bump syn from 2.0.108 to 2.0.110
- *(deps)* bump insta from 1.43.2 to 1.44.0
- *(deps)* bump serde_with from 3.15.1 to 3.16.0
- *(deps)* bump indexmap from 2.11.4 to 2.12.0
- Acknowledge recent change in output data
- Generated Rust files: Exempt from rustfmt
- Accept Clippy fix
- *(deps)* bump lazy-regex from 3.4.1 to 3.4.2
- *(deps)* bump yaml-hash from 0.5.0 to 0.5.1
- *(deps)* bump syn from 2.0.107 to 2.0.108
- *(deps)* bump serde_with from 3.15.0 to 3.15.1
- *(deps)* bump syn from 2.0.106 to 2.0.107
- *(deps)* bump toml from 0.9.7 to 0.9.8

## [0.1.9](https://github.com/ariel-os/sbd/compare/v0.1.8...v0.1.9) - 2025-10-10

### Fixed

- *(ariel)* don't create `todo!()` in cfg-if macro case

## [0.1.8](https://github.com/ariel-os/sbd/compare/v0.1.7...v0.1.8) - 2025-10-10

### Fixed

- *(ariel)* don't emit doc comments

## [0.1.7](https://github.com/ariel-os/sbd/compare/v0.1.6...v0.1.7) - 2025-10-09

### Fixed

- *(ariel)* fix cfg-if board dispatch
- *(gen)* add newline to `.sbd-gen` tag file

## [0.1.6](https://github.com/ariel-os/sbd/compare/v0.1.5...v0.1.6) - 2025-10-09

### Added

- *(ariel)* add @generated comment to Cargo.toml
- *(ariel)* format Rust code with prettyplease
- *(cli)* add `--version`, handle no subcommand

### Other

- *(sbd)* drop file-level `description` field
- *(ariel)* drop some manual .rs formatting
- *(deps)* bump serde_with from 3.14.1 to 3.15.0
- *(deps)* bump serde_with from 3.14.0 to 3.14.1
- *(deps)* bump camino from 1.2.0 to 1.2.1
- *(deps)* bump serde from 1.0.227 to 1.0.228

## [0.1.5](https://github.com/ariel-os/sbd/compare/v0.1.4...v0.1.5) - 2025-09-27

### Fixed

- *(ariel)* drop context conditionals in per-board files
- make pin2tuple regex lazy

## [0.1.4](https://github.com/ariel-os/sbd/compare/v0.1.3...v0.1.4) - 2025-09-26

### Fixed

- uninlined-format-args in anyhow use
- don't consider tagfile as extra file

### Other

- *(ariel)* clarify `--mode` options
- *(deps)* bump anyhow from 1.0.99 to 1.0.100
- *(deps)* bump yaml-hash from 0.4.5 to 0.5.0
- *(deps)* bump camino from 1.1.11 to 1.2.0
- *(deps)* bump regex from 1.11.2 to 1.11.3
- *(deps)* bump toml from 0.9.5 to 0.9.7

## [0.1.3](https://github.com/ariel-os/sbd/compare/v0.1.2...v0.1.3) - 2025-09-26

### Fixed

- remove debug leftovers
- typos sbg -> sbd

### Other

- more cleanup

## [0.1.2](https://github.com/ariel-os/sbd/compare/v0.1.1...v0.1.2) - 2025-09-26

### Added

- add snapshot tests

### Fixed

- *(ariel)* use BTreeMap for crate Manifest tables

## [0.1.1](https://github.com/ariel-os/sbd/compare/v0.1.0...v0.1.1) - 2025-09-26

### Added

- implement `check` mode
- FileMap compare

### Other

- newtype FileMap
## [0.1.0] - 2025-09-26

### 🚀 Features

- Initial RIOT support
- Update Crate manifest edition/rust-version handling
- *(ariel)* Add header comment with yamllint ignore to laze file
- *(ariel)* Add `--overwrite` flag
- *(ariel)* Introduce StringOrWorkspace

### 🐛 Bug Fixes

- *(ariel)* Allow unused variables / imports`
- *(ariel)* Add `// @generated` to generated rust files

### 🚜 Refactor

- Introduce generate-ariel subcommand
- *(ariel)* Create per-board .rs files
- Soc -> chip
- *(ariel)* Introduce `--mode`
- Factor out file writing
- Factor out file writing (#6)

### 📚 Documentation

- Add README.md

### ⚙️ Miscellaneous Tasks

- Add initial dependabot config
- Add Rust build workflow
- Fix some lints
- Run clippy
- Fix clippy pedantic
- Rename to sbd-gen
- Update Cargo.toml
- Add container workflow
- Add container workflow (#7)
- Fix container workflow
- Fix container workflow (#8)
- Bump buster rust version
- Release-plz initial
- Release-plz initial (#3)
- Fix releaze-plz app id arg
