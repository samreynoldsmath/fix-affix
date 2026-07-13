use std::path::{Path, PathBuf};

pub struct ContextManager {
    pub in_file: PathBuf,
    pub out_dir: PathBuf,
}

impl ContextManager {
    pub fn new(args: Vec<String>) -> Self {
        if args.len() < 2 {
            panic!("Please supply input file");
        }

        let in_file: PathBuf = Path::new(&args[1]).to_owned();
        let out_dir: PathBuf = Path::new(&args[2]).to_owned();

        if !out_dir.exists()
            && let Err(e) = std::fs::create_dir(&out_dir)
        {
            panic!("Unable to create directory ({:?}): {}", out_dir, e)
        }
        if !out_dir.is_dir() {
            panic!("The output path ({:?}) must be a directory", out_dir);
        }

        ContextManager { in_file, out_dir }
    }
}
