# `sbd-gen-schema`

This crate defines the schema used by `sbd-gen`.

## Versioning

The sbd schema is versioned in order to check for compatibility between sbd
files and the schema version used by the parser.

Each sbd file should contain a `version: a.b.c` field that corresponds to the
schema version as used by `sbd-gen`. If no version is specified, the version of
a file defaults to `0.1.0`.

To get the currently used schema version, run `sbd-gen --version`.

Versions are compared using [Cargo's semver rules][cargo-semver].
The file version specifies a minimum version with the ability to update to
SemVer-compatible versions. Versions are considered compatible if their
left-most non-zero major/minor/patch component is the same. (This is different from [SemVer](https://semver.org/) which considers all pre-1.0.0 versions to be incompatible.)

E.g., the following file versions would match the following schema versions:

    2.0.0 := >=2.0.0, <3.0.0
    1.2.3 := >=1.2.3, <2.0.0
    1.2.0 := >=1.2.0, <2.0.0
    0.2.3 := >=0.2.3, <0.3.0
    0.0.3 := >=0.0.3, <0.0.4

[cargo-semver]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#default-requirements
