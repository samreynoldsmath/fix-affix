use std::path::{Path, PathBuf};

pub struct ContextManager {
    pub toml_file: PathBuf,
    pub aff_file: PathBuf,
    pub dic_file: PathBuf,
}

impl ContextManager {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() < 2 {
            panic!("Please supply input file");
        }
        let toml_file: PathBuf = Path::new(&args[1]).to_owned();
        let aff_file: PathBuf = toml_file.with_extension("aff");
        let dic_file: PathBuf = toml_file.with_extension("dic");
        ContextManager {
            toml_file,
            aff_file,
            dic_file,
        }
    }
}
