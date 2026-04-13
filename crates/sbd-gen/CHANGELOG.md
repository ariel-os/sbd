# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/ariel-os/sbd/compare/sbd-gen-v0.3.0...sbd-gen-v0.3.1) - 2026-04-08

### Other

- *(deps)* bump insta from 1.47.1 to 1.47.2
- *(deps)* bump insta from 1.47.0 to 1.47.1
- *(deps)* bump insta from 1.46.3 to 1.47.0
- *(uart)* equate absent and empty .possible_peripherals
- Test code generation
- Generate code for UARTs
- Expose presence of host facing UARTs as `has_host_facing_uart`
- address clippy lint

## [0.3.0](https://github.com/ariel-os/sbd/compare/sbd-gen-v0.2.0...sbd-gen-v0.3.0) - 2026-03-20

### Other

- [**breaking**] rename `boards` key to `targets`

## [0.2.0](https://github.com/ariel-os/sbd/compare/sbd-gen-v0.1.12...sbd-gen-v0.2.0) - 2026-03-11

### Other

- bump sbd-gen version to 0.2.0 due to breaking schema change

## [0.1.12](https://github.com/ariel-os/sbd/compare/sbd-gen-v0.1.9...sbd-gen-v0.1.12) - 2026-03-10

### Added

- initial schema versioning
- *(schema)* update button and led `aliases` and `active`
- provide better package metadata

### Other

- *(sbd)* bump version
- release
- *(schema)* bump to `0.2.0`
- fix clippy lints
- create cargo workspace and move schema in separate crate

## [0.1.11](https://github.com/ariel-os/sbd/compare/sbd-gen-v0.1.9...sbd-gen-v0.1.11) - 2026-03-10

### Added

- initial schema versioning
- *(schema)* update button and led `aliases` and `active`
- provide better package metadata

### Other

- *(schema)* bump to `0.2.0`
- fix clippy lints
- create cargo workspace and move schema in separate crate
