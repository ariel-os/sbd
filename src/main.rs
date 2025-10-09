use sbd::SbdFile;
use walkdir::WalkDir;
use yaml_hash::YamlHash;

mod ariel;
mod filemap;
mod krate;
mod laze;
mod pin2tuple;
mod riot;
mod sbd;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(argh::FromArgs, Debug)]
#[argh(description = "SDB file parser")]
struct Args {
    /// change working directory before doing anything else
    #[argh(option, short = 'C')]
    chdir: Option<String>,

    /// print version and exit
    #[argh(switch, short = 'V')]
    version: bool,

    #[argh(subcommand)]
    subcommand: Option<Subcommands>,
}

#[derive(argh::FromArgs, Debug)]
#[argh(subcommand)]
enum Subcommands {
    GenerateAriel(ariel::GenerateArielArgs),
    GenerateRiot(riot::GenerateRiotArgs),
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    if args.version {
        println!("sbd version {VERSION}");
        return Ok(());
    }

    if let Some(dir) = args.chdir.as_ref() {
        println!("sbd: changing to '{dir}'");
        std::env::set_current_dir(dir)?;
    }

    match args.subcommand {
        Some(Subcommands::GenerateAriel(args)) => ariel::generate(&args)?,
        Some(Subcommands::GenerateRiot(args)) => riot::generate(&args)?,
        None => {
            println!("sbd: no subcommand given. try `sbd-gen --help`.");
        }
    }
    Ok(())
}

fn parse_sbd_files(sbd_dir: &str) -> anyhow::Result<SbdFile> {
    // Walk through the directory, collect all files ending with `.yaml`.
    // Then sort that list.
    let mut files = Vec::new();
    for entry in WalkDir::new(sbd_dir)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            std::path::Path::new(e.file_name().to_str().unwrap())
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("yaml"))
        })
    {
        files.push(entry.path().to_str().unwrap().to_string());
    }
    files.sort();

    // Merge all the files into a single yaml object.
    let mut hash = YamlHash::new();
    for file in files {
        println!("sbd: processing '{file}'");
        hash = hash.merge_file(&file)?;
    }

    // Now do magic: serialize again, then deserialize into our known type.
    let merged = hash.to_string();
    let sbd_file: SbdFile = serde_yaml::from_str(&merged).unwrap();

    Ok(sbd_file)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbd_ariel() {
        let sbd_file = parse_sbd_files("sbd-test-files").unwrap();
        let ariel = ariel::render_ariel_board_crate(&sbd_file);
        insta::assert_debug_snapshot!(ariel);
    }
}
