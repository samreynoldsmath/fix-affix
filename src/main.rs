use fix_affix::HunspellDict;
use std::env::args;
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Please supply input file");
    }

    let toml_file: PathBuf = Path::new(&args[1]).to_owned();
    let aff_file: PathBuf = toml_file.with_extension("aff");
    let dic_file: PathBuf = toml_file.with_extension("dic");

    let dict: HunspellDict = match HunspellDict::load_from_toml_file(&toml_file) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", &toml_file, e),
    };

    if let Err(e) = dict.write_dic_file(&dic_file) {
        panic!("Failed to build Hunspell dic: {}", e)
    };

    if let Err(e) = dict.write_aff_file(&aff_file) {
        panic!("Failed to build Hunspell aff: {}", e)
    };
}
