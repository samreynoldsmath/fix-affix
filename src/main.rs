use fix_affix::{build_hunspell_dictionary, load_toml_dict};
use std::env::args;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        panic!("Please supply input file and output path");
    }
    let in_path: &Path = Path::new(&args[1]);
    let out_path: &Path = Path::new(&args[2]);
    let dict: fix_affix::TomlDict = match load_toml_dict(in_path) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", in_path, e),
    };
    if !out_path.exists()
        && let Err(e) = fs::create_dir(out_path)
    {
        panic!("Unable to create directory ({:?}): {}", out_path, e)
    }
    if !out_path.is_dir() {
        panic!("The output path ({:?}) must be a directory", out_path);
    }
    if let Err(e) = build_hunspell_dictionary(out_path, &dict) {
        panic!("Failed to build Hunspell dictionary: {}", e)
    }
}
