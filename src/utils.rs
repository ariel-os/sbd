use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};

pub fn write_all<'a>(
    base_path: &'a Utf8Path,
    files: impl Iterator<Item = (&'a Utf8PathBuf, &'a String)>,
) -> Result<()> {
    for (file, content) in files {
        let file_path = base_path.join(file);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file_path, content)?;
    }

    Ok(())
}
