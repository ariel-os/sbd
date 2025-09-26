# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
