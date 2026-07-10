use fix_affix::load_toml_dict;
use std::env::args;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        panic!("Please supply input and output paths");
    }
    let in_path: &Path = Path::new(&args[1]);
    let out_path: &Path = Path::new(&args[2]);
    let dict: fix_affix::TomlDict = match load_toml_dict(in_path) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", in_path, e),
    };
    dbg!(&dict);
    dbg!(out_path);
}
