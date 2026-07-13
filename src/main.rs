use fix_affix::{ContextManager, TomlDict, build_hunspell_dictionary, load_toml_dict};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let ctx: ContextManager = ContextManager::new(args);
    let dict: TomlDict = match load_toml_dict(&ctx.in_file) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", &ctx.in_file, e),
    };
    if let Err(e) = build_hunspell_dictionary(&ctx.out_dir, &dict) {
        panic!("Failed to build Hunspell dictionary: {}", e)
    }
}
