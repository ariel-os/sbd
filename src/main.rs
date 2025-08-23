use sbd::SbdFile;
use walkdir::WalkDir;
use yaml_hash::YamlHash;

mod ariel;
mod krate;
mod laze;
mod sbd;

#[derive(argh::FromArgs, Debug)]
#[argh(description = "SDB file parser")]
struct Args {
    /// change working directory before doing anything else
    #[argh(option, short = 'C')]
    chdir: Option<String>,

    /// the name of the directory containing board descriptions
    #[argh(positional)]
    sbd_dir: String,

    /// ariel os boards crate output folder
    #[argh(option, short = 'o', default = "String::from(\"ariel-os-boards\")")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    if let Some(dir) = args.chdir.as_ref() {
        println!("sbd: changing to '{dir}'");
        std::env::set_current_dir(dir)?;
    }

    // Walk through the directory, collect all files ending with `.yaml`.
    // Then sort that list.
    let mut files = Vec::new();
    for entry in WalkDir::new(args.sbd_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str().unwrap().ends_with(".yaml"))
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

    // Finally, render the ariel crate.
    ariel::render_ariel_board_crate(&sbd_file, args.output.as_str().into())?;

    Ok(())
}
