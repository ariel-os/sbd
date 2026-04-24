# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/ariel-os/sbd/compare/sbd-gen-schema-v0.3.0...sbd-gen-schema-v0.4.0) - 2026-04-08

### Added

- make schema version separate from schema crate version

### Other

- *(deps)* bump semver from 1.0.27 to 1.0.28
- *(uart)* equate absent and empty .possible_peripherals
- Generate code for UARTs
- Add property for UART MCU peripherals
- Expose presence of host facing UARTs as `has_host_facing_uart`
- Define host_facing UART property

## [0.3.0](https://github.com/ariel-os/sbd/compare/sbd-gen-schema-v0.2.0...sbd-gen-schema-v0.3.0) - 2026-03-20

### Other

- [**breaking**] rename `boards` key to `targets`
- [**breaking**] rename `RiotQirkEntry` to `RiotQuirkEntry`

## [0.2.0](https://github.com/ariel-os/sbd/compare/sbd-gen-schema-v0.1.0...sbd-gen-schema-v0.2.0) - 2026-03-10

### Added

- initial schema versioning
- *(schema)* update button and led `aliases` and `active`

### Fixed

- *(schema)* change the default version to `0.2.0`

### Other

- *(schema)* bump to `0.2.0`
