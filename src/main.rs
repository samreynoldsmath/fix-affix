use fix_affix::{ContextManager, TomlDict, build_hunspell_dictionary, load_toml_dict};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let ctx: ContextManager = ContextManager::new(args);
    let dict: TomlDict = match load_toml_dict(&ctx.toml_file) {
        Ok(data) => data,
        Err(e) => panic!("TOML dictionary not loaded ({:?}): {}", &ctx.toml_file, e),
    };
    if let Err(e) = build_hunspell_dictionary(&dict, &ctx.aff_file, &ctx.dic_file) {
        panic!("Failed to build Hunspell dictionary: {}", e)
    }
}
