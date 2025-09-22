use std::collections::HashMap;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct Crate {
    pub manifest: Manifest,
    pub files: HashMap<Utf8PathBuf, String>,
}

impl Crate {
    pub fn new(name: &str) -> Self {
        Self {
            manifest: Manifest::new(name),
            ..Default::default()
        }
    }

    pub fn write_to_directory(&self, path: &Utf8Path, overwrite: bool) -> Result<()> {
        let manifest_path = path.join("Cargo.toml");

        if !overwrite && path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Directory already exists",
            )
            .into());
        }

        std::fs::create_dir_all(path)?;

        let manifest_content = toml::to_string(&self.manifest).unwrap();
        std::fs::write(manifest_path, manifest_content)?;

        crate::utils::write_all(path, self.files.iter())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: HashMap<String, Dependency>,
    pub features: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum Dependency {
    #[default]
    Default,
    Version(String),
    Full(DependencyFull),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrWorkspace {
    String(String),
    Workspace(HashMap<String, bool>),
}

impl StringOrWorkspace {
    pub fn workspace() -> Self {
        Self::Workspace(HashMap::from([("workspace".into(), true)]))
    }
    #[expect(dead_code, reason = "currently unused, unmark when it is")]
    pub fn string(string: &str) -> Self {
        Self::String(string.into())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DependencyFull {
    pub version: Option<String>,
    pub package: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub optional: Option<bool>,
    pub default_features: Option<bool>,
    pub features: Option<Vec<String>>,
    pub workspace: Option<bool>,
}

impl Manifest {
    pub fn new(name: &str) -> Self {
        Self {
            package: Package::new(name),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub edition: Option<StringOrWorkspace>,
    pub license: Option<StringOrWorkspace>,
    #[serde(rename = "rust-version")]
    pub rust_version: Option<StringOrWorkspace>,
}

pub fn workspace() -> Option<StringOrWorkspace> {
    Some(StringOrWorkspace::workspace())
}

impl Package {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}
