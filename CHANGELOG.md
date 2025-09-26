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
